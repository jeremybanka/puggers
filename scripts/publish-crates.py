"""Publish only confirmed missing versions; registry uncertainty stops the release."""

import argparse
import json
import re
import subprocess
import sys
import time


class RegistryError(RuntimeError):
    pass


def is_published(crate, version):
    name = crate.lower()
    if not re.fullmatch(r"[a-z0-9_-]+", name):
        raise RegistryError(f"Invalid crate name: {crate!r}")
    if len(name) == 1:
        path = f"1/{name}"
    elif len(name) == 2:
        path = f"2/{name}"
    elif len(name) == 3:
        path = f"3/{name[0]}/{name}"
    else:
        path = f"{name[:2]}/{name[2:4]}/{name}"
    url = f"https://index.crates.io/{path}"
    result = subprocess.run(
        ["curl", "--silent", "--show-error", "--location",
         "--proto", "=https", "--proto-redir", "=https",
         "--connect-timeout", "10", "--max-time", "30",
         "--user-agent", "puggers-release (https://github.com/jeremybanka/puggers)",
         "--header", "Cache-Control: no-cache", "--write-out", "\n%{http_code}", url],
        capture_output=True, text=True, timeout=35,
    )
    if result.returncode:
        raise RegistryError(
            f"Cannot check {crate}@{version}: curl exit {result.returncode} at {url}: "
            f"{result.stderr.strip()}"
        )
    body, _, status = result.stdout.rpartition("\n")
    if status == "404":
        return False
    if status != "200":
        raise RegistryError(f"Cannot check {crate}@{version}: HTTP {status!r} at {url}")
    try:
        records = [json.loads(line) for line in body.splitlines()]
        if not records or any(
            not isinstance(record, dict)
            or record.get("name") != crate
            or not isinstance(record.get("vers"), str)
            or not isinstance(record.get("yanked"), bool)
            for record in records
        ):
            raise ValueError("empty index or invalid crate/version/yanked fields")
        exact = [record for record in records if record["vers"] == version]
        if len(exact) > 1:
            raise ValueError("duplicate exact version")
        if exact and exact[0]["yanked"]:
            raise ValueError("exact version is yanked; manual review required")
    except (ValueError, TypeError) as error:
        raise RegistryError(f"Cannot check {crate}@{version}: invalid index at {url}: {error}") from error
    return bool(exact)


def cargo(crate, *, dry_run):
    args = ["cargo", "publish", "-p", crate, "--locked"]
    if dry_run:
        args.append("--dry-run")
    subprocess.run(args, check=True)


def wait_for_crate(crate, version):
    for attempt in range(1, 21):
        if is_published(crate, version):
            print(f"Verified {crate}@{version} in the crates.io sparse index.", flush=True)
            return
        print(f"Waiting for {crate}@{version} in the sparse index ({attempt}/20)...", flush=True)
        if attempt < 20:
            time.sleep(15)
    raise RegistryError(f"Timed out waiting for {crate}@{version} in the crates.io sparse index.")


def publish(version, crates, *, dry_run=False, require_core=False):
    if require_core and not is_published("puggers-core", version):
        raise RegistryError(f"Recovery requires puggers-core@{version} to be published and non-yanked.")
    # Check every registry response before the first possible upload.
    statuses = {crate: is_published(crate, version) for crate in crates}
    for crate, present in statuses.items():
        if present or is_published(crate, version):
            print(f"{crate}@{version} is already published; skipping.", flush=True)
            continue
        cargo(crate, dry_run=True)
        if not dry_run:
            cargo(crate, dry_run=False)
            wait_for_crate(crate, version)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["check", "publish", "preflight"])
    parser.add_argument("version")
    parser.add_argument("crates", nargs="+")
    parser.add_argument("--require-core", action="store_true")
    args = parser.parse_args()
    try:
        if args.mode == "check":
            statuses = [is_published(crate, args.version) for crate in args.crates]
            for crate, present in zip(args.crates, statuses):
                print(f"{crate}@{args.version}: {'present' if present else 'absent'}")
            return 0 if all(statuses) else 1
        publish(args.version, args.crates, dry_run=args.mode == "preflight", require_core=args.require_core)
    except (RegistryError, subprocess.SubprocessError, OSError) as error:
        print(f"Release stopped: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())

import importlib.util
import json
from pathlib import Path
import subprocess
import unittest
from unittest.mock import patch


spec = importlib.util.spec_from_file_location(
    "publish_crates", Path(__file__).parents[1] / "publish-crates.py"
)
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


def response(status=200, records=None, body=None, returncode=0):
    if body is None:
        body = "\n".join(json.dumps(record) for record in (records or []))
    return subprocess.CompletedProcess(
        [], returncode, f"{body}\n{status}", "connection failed" if returncode else ""
    )


def record(version="0.1.12", **overrides):
    return {"name": "puggers-core", "vers": version, "yanked": False, **overrides}


class RegistryTests(unittest.TestCase):
    def probe(self, result):
        with patch.object(release.subprocess, "run", return_value=result):
            return release.is_published("puggers-core", "0.1.12")

    def test_exact_non_yanked_version_is_present(self):
        self.assertTrue(self.probe(response(records=[record("0.1.8"), record()])))

    def test_other_versions_are_absent(self):
        self.assertFalse(self.probe(response(records=[record("0.1.8")])))

    def test_missing_crate_is_absent(self):
        self.assertFalse(self.probe(response(status=404)))

    def test_http_and_transport_errors_are_uncertain(self):
        for result in [response(status=403), response(status=429), response(status=503),
                       response(returncode=28), response(returncode=60)]:
            with self.subTest(result=result), self.assertRaises(release.RegistryError):
                self.probe(result)

    def test_malformed_or_empty_index_is_uncertain(self):
        for result in [response(body="<html>blocked</html>"), response(),
                       response(records=[record(), {"error": "bad"}]),
                       response(records=[record(name="other")]),
                       response(records=[record(yanked="false")])]:
            with self.subTest(result=result), self.assertRaises(release.RegistryError):
                self.probe(result)

    def test_yanked_and_duplicate_exact_versions_are_uncertain(self):
        for records in [[record(yanked=True)], [record(), record()]]:
            with self.subTest(records=records), self.assertRaises(release.RegistryError):
                self.probe(response(records=records))

    def test_official_sparse_url_and_bounded_request(self):
        with patch.object(release.subprocess, "run", return_value=response(status=404)) as run:
            release.is_published("puggers-core", "0.1.12")
        args = run.call_args.args[0]
        self.assertIn("https://index.crates.io/pu/gg/puggers-core", args)
        self.assertIn("--show-error", args)
        self.assertIn("--max-time", args)
        self.assertNotIn("--fail", args)


class PublishingTests(unittest.TestCase):
    def test_uncertainty_before_publish_runs_no_cargo(self):
        with patch.object(release, "is_published", side_effect=[True, release.RegistryError("HTTP 503")]), \
             patch.object(release, "cargo") as cargo:
            with self.assertRaises(release.RegistryError):
                release.publish("0.1.12", ["puggers-core", "puggers"])
            cargo.assert_not_called()

    def test_already_published_crate_is_skipped(self):
        with patch.object(release, "is_published", return_value=True), \
             patch.object(release, "cargo") as cargo:
            release.publish("0.1.12", ["puggers-core"])
            cargo.assert_not_called()

    def test_missing_crate_is_dry_run_checked_then_published_and_verified(self):
        with patch.object(release, "is_published", side_effect=[False, False, True]), \
             patch.object(release, "cargo") as cargo:
            release.publish("0.1.12", ["puggers"])
        self.assertEqual(cargo.call_args_list, [
            unittest.mock.call("puggers", dry_run=True),
            unittest.mock.call("puggers", dry_run=False),
        ])

    def test_preflight_cannot_publish(self):
        with patch.object(release, "is_published", side_effect=[True, False, False, False, False]), \
             patch.object(release, "cargo") as cargo:
            release.publish("0.1.12", ["puggers", "dprint-plugin-pug"], dry_run=True, require_core=True)
        self.assertEqual(cargo.call_args_list, [
            unittest.mock.call("puggers", dry_run=True),
            unittest.mock.call("dprint-plugin-pug", dry_run=True),
        ])

    def test_recovery_refuses_missing_core(self):
        with patch.object(release, "is_published", return_value=False), \
             patch.object(release, "cargo") as cargo:
            with self.assertRaises(release.RegistryError):
                release.publish("0.1.12", ["puggers"], require_core=True)
            cargo.assert_not_called()

    def test_dry_run_failure_prevents_upload(self):
        with patch.object(release, "is_published", return_value=False), \
             patch.object(release, "cargo", side_effect=subprocess.CalledProcessError(1, "cargo")) as cargo:
            with self.assertRaises(subprocess.CalledProcessError):
                release.publish("0.1.12", ["puggers"])
            self.assertEqual(cargo.call_count, 1)

    def test_wait_retries_only_confirmed_absence(self):
        with patch.object(release, "is_published", side_effect=[False, True]), \
             patch.object(release.time, "sleep") as sleep:
            release.wait_for_crate("puggers-core", "0.1.12")
            sleep.assert_called_once_with(15)

    def test_wait_request_failure_stops_immediately(self):
        with patch.object(release, "is_published", side_effect=release.RegistryError("HTTP 403")), \
             patch.object(release.time, "sleep") as sleep:
            with self.assertRaises(release.RegistryError):
                release.wait_for_crate("puggers-core", "0.1.12")
            sleep.assert_not_called()

    def test_wait_has_a_bounded_timeout(self):
        with patch.object(release, "is_published", return_value=False) as probe, \
             patch.object(release.time, "sleep") as sleep:
            with self.assertRaises(release.RegistryError):
                release.wait_for_crate("puggers-core", "0.1.12")
            self.assertEqual(probe.call_count, 20)
            self.assertEqual(sleep.call_count, 19)


if __name__ == "__main__":
    unittest.main()

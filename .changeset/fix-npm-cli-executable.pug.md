---
default: patch
---

### Fix npm CLI executable setup

Prepare a top-level native `puggers` executable during npm install so the CLI runs even when native package tarballs do not preserve executable mode bits.

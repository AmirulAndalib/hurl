#!/bin/bash
set -Eeuo pipefail

hurl --repeat 4 \
  tests_ok/repeat_a.hurl \
  tests_ok/repeat_b.hurl \
  tests_ok/repeat_c.hurl

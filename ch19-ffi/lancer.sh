#!/usr/bin/env bash
# Compile le module Python `capteurs` (maturin), lance les tests Python puis le benchmark.
set -euo pipefail
cd "$(dirname "$0")"
[ -x .venv/bin/python ] || python3 -m venv .venv
# shellcheck disable=SC1091
. .venv/bin/activate
pip install -q maturin numpy
maturin develop --release -q
python -m unittest discover -s tests -v
python bench.py

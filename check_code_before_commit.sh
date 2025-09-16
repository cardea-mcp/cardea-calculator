#!/bin/bash

# check code linting
cargo +nightly clippy --all-features -- -D warnings

# check code formatting
cargo +nightly fmt --all -- --check
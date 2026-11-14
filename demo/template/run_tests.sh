#!/bin/sh
# Runs the visible test suite of this migration task.
cargo test --quiet --test visible 2>&1 | tail -40

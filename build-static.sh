#!/bin/sh

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

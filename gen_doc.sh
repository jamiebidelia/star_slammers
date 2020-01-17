#!/bin/sh

# RustDoc will not normally provide documentation for a binary.  I found this on a discussion at
# https://github.com/rust-lang/cargo/issues/1865
# and it seems to do the job.

cargo rustdoc --open -- --no-defaults \
  --passes collapse-docs   \
  --passes unindent-comments \
  --passes strip-priv-imports

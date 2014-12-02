#!/bin/sh

rustc cgol.rs
./cgol $(tput cols) $(tput lines)

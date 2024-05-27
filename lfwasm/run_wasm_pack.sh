#!/bin/bash
wasm-pack build . || npx -y wasm-pack build .

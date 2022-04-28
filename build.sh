#!/bin/bash
app='toolbox'
yarn tauri:build
#upx --ultra-brute src-tauri/target/release/$app
mv src-tauri/target/release/$app ~/mybash/workshell/rt
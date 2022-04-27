#!/bin/bash
app='toolbox'
yarn tauri:build
#upx --ultra-brute src-tauri/target/release/$app
cp -r src-tauri/target/release/$app ~/Desktop/
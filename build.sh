#!/bin/bash
app='toolbox'
yarn tauri:build
upx --ultra-brute src-tauri/target/release/bundle/macos/$app.app/Contents/MacOS/$app
cp -r src-tauri/target/release/bundle/macos/$app.app ~/Desktop/
#!/usr/bin/env bash

cargo install tauri-cli

wget https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip -O cmdline-tools.zip
unzip cmdline-tools.zip
mkdir ~/.android
mkdir ~/.android/cmdline-tools
mv cmdline-tools ~/.android/cmdline-tools/tools
~/.android/cmdline-tools/tools/bin/sdkmanager "platforms;android-33" "platform-tools" "ndk;25.0.8775105" "build-tools;33.0.0"
rm cmdline-tools.zip
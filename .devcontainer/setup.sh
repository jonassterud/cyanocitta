#!/usr/bin/env bash

cargo install tauri-cli --version "^2.0.0-alpha"

export JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64
export ANDROID_HOME=$HOME/.android
export NDK_HOME=$ANDROID_HOME/ndk/25.0.8775105

wget https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip -O cmdline-tools.zip
unzip cmdline-tools.zip
mkdir ~/.android
mkdir ~/.android/cmdline-tools
mv cmdline-tools ~/.android/cmdline-tools/tools
~/.android/cmdline-tools/tools/bin/sdkmanager "platforms;android-33" "platform-tools" "ndk;25.0.8775105" "build-tools;33.0.0"
rm cmdline-tools.zip

cargo tauri android init
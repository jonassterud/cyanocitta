ARG VARIANT="jammy"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

ARG TAURI_BUILD_DEPS="build-essential curl libappindicator3-dev libgtk-3-dev librsvg2-dev libssl-dev libwebkit2gtk-4.0-dev wget"
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends $TAURI_BUILD_DEPS
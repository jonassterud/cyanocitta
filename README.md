# Cyanocitta

[![CI](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml)
[![Release](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml)

## Contributing
Feel free to contribute!

Use tools such as [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to improve your code.  
Commit messages should follow [conventionalcommits.org](https://www.conventionalcommits.org).  
Where type is one of the following: `feat`, `fix`, `ci`, `docs` or `refactor`.

### Debugging wirelessly on Android devices
You can use the ["Dev containers"](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension for VS Code to develop using the `.devcontainer` which will install the necessary tools.

Here is how you can wirelessly connect to your Android phone:

* On your Android device, go into `Developer settings > Wireless debugging > Enable`.
* Click on "Pair device using code".
* Inside the directory `~/.android/platform-tools/` you'll find the `adb` executable.
* Use this to pair with you Android device: `adb pair <PAIR_IP:PAIR_PORT> <CODE>`.
* Now run: `adb connect <IP:PORT>`.
* Check if pairing was successful by running: `adb devices`.
* Now run: `cargo tauri android init`.
* Now run: `cargo tauri android dev`.

Oops - this doesn't work yet, see issue [#1](https://github.com/jonassterud/cyanocitta/issues/1).

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.

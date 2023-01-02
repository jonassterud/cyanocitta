# Cyanocitta

[![CI](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml)
[![Release](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml)

# Supported NIPs
- [x] [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md)
- [ ] [NIP-02](https://github.com/nostr-protocol/nips/blob/master/02.md)
- [ ] [NIP-03](https://github.com/nostr-protocol/nips/blob/master/03.md)
- [ ] [NIP-04](https://github.com/nostr-protocol/nips/blob/master/04.md)
- [ ] [NIP-05](https://github.com/nostr-protocol/nips/blob/master/05.md)
- [ ] [NIP-06](https://github.com/nostr-protocol/nips/blob/master/06.md)
- [ ] [NIP-07](https://github.com/nostr-protocol/nips/blob/master/07.md)
- [ ] [NIP-08](https://github.com/nostr-protocol/nips/blob/master/08.md)
- [ ] [NIP-09](https://github.com/nostr-protocol/nips/blob/master/09.md)
- [ ] [NIP-10](https://github.com/nostr-protocol/nips/blob/master/10.md)
- [x] [NIP-11](https://github.com/nostr-protocol/nips/blob/master/11.md)
- [ ] [NIP-12](https://github.com/nostr-protocol/nips/blob/master/12.md)
- [ ] [NIP-13](https://github.com/nostr-protocol/nips/blob/master/13.md)
- [ ] [NIP-14](https://github.com/nostr-protocol/nips/blob/master/14.md)
- [ ] [NIP-15](https://github.com/nostr-protocol/nips/blob/master/15.md)
- [ ] [NIP-16](https://github.com/nostr-protocol/nips/blob/master/16.md)
- [ ] [NIP-19](https://github.com/nostr-protocol/nips/blob/master/19.md)
- [ ] [NIP-20](https://github.com/nostr-protocol/nips/blob/master/20.md)
- [ ] [NIP-22](https://github.com/nostr-protocol/nips/blob/master/22.md)
- [ ] [NIP-25](https://github.com/nostr-protocol/nips/blob/master/25.md)
- [ ] [NIP-26](https://github.com/nostr-protocol/nips/blob/master/26.md)
- [ ] [NIP-28](https://github.com/nostr-protocol/nips/blob/master/28.md)
- [ ] [NIP-35](https://github.com/nostr-protocol/nips/blob/master/35.md)
- [ ] [NIP-36](https://github.com/nostr-protocol/nips/blob/master/36.md)
- [ ] [NIP-40](https://github.com/nostr-protocol/nips/blob/master/40.md)

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

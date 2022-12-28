# Cyanocitta

[![CI](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/ci.yml)
[![Release](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml/badge.svg)](https://github.com/jonassterud/cyanocitta/actions/workflows/release.yml)

## Contributing
Feel free to contribute!

Use tools such as [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to improve your code.  
Commit messages should follow [conventionalcommits.org](https://www.conventionalcommits.org).  
Where type is one of the following: `feat`, `fix`, `ci`, `docs` or `refactor`.

### Devcontainer
You can use the ["Dev containers"](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension for VS Code to develop using the `.devcontainer`.  
Here are some things you might need to do:

* You need to manually set the `DISPLAY` variable in `devcontainer.json`.
* You might have to manually define the variables in `devcontainer.env` inside the container.
* For virtualization to work, here are some things you might have to do:

    * Run: `sudo groupadd -r kvm`.
    * Ensure `/lib/udev/rules.d/50-udev-default.rules` contains something like: `KERNEL=="kvm", GROUP="kvm", MODE="0660"`.
    * Run: `sudo gpasswd -a $USER kvm`.
    * Run: `sudo chown $USER /dev/kvm`.
    * You might have to log out and back in.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.

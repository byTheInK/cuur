# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation
You can get Cuur from [here](https://github.com/byTheInK/cuur/releases).

## Building from source

# Examples

Install `vim` and `htop` in all of the distributions.
```toml
[sys]
works_on = ["all"]

[pkg]
install = ["vim", "htop"]
```

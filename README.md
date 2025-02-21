# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation
Coming soon...

# Examples

Install `vim` and `htop` in all of the distros.
```toml
[sys]
works_on = ["all"]

[pkg]
install = ["vim", "htop"]
```

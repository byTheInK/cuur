# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation

## Building from source
First of all you have to ensure you have the necessary packages.

### Debian based systems:
```bash
sudo apt install cargo rustup git
```

### RHEL based systems:
```bash
sudo dnf install cargo rustup git
```

### Arch based systems:
```bash
sudo pacman -S cargo rustup git
```

# Examples

Install `vim` and `htop` in all of the distributions.
```toml
[sys]
works_on = ["all"]

[pkg]
install = ["vim", "htop"]
```

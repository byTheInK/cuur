# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation

## Building from source
First of all you have to ensure you have the necessary packages.

### Debian based systems:
```bash
sudo apt install cargo git
```

### RHEL based systems:
```bash
sudo dnf install cargo git
```

### Arch based systems:
```bash
sudo pacman -S cargo git
```

Now, we can clone the repository.
```bash
git clone https://github.com/byTheInK/cuur
cd cuur
```

Let's install the building tools.
```bash
cargo install cargo-deb
cargo install cargo-rpm
cargo install cargo-generate
cargo install cargo-bundle
```

Now, we can build the binary with Cargo.
```bash
cargo build
```


# Examples

Install `vim` and `htop` in all of the distributions.
```toml
[sys]
works_on = ["all"]

[pkg]
install = ["vim", "htop"]
```

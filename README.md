# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation

## Building from source
First of all you have to ensure you have the necessary packages. Make sure you have curl installed.

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Debian based systems:
```bash
sudo apt install cargo git rpm python-pipx
```

### RHEL based systems:
```bash
sudo dnf install cargo git rpm-build python-pipx
```

### Arch based systems:
```bash
sudo pacman -S cargo git rpm python-pipx base base-devel
```

Now, we can clone the repository.
```bash
git clone https://github.com/byTheInK/cuur
cd cuur
```

Let's install the building tools. Pick the tool or tools you want.
```bash
cargo install cargo-deb # Debian
cargo install cargo-rpm # RHEL
cargo install cargo-aur # Arch
cargo install cargo-bundle # AppImage
```

### Debian package
```bash
cargo rpm build
```
You can find the package in the `target/debian/` directory. To install we can use the command below.

```bash
cd target/debian/
sudo dpkg -i *.deb
```

### Rpm package
```bash
cargo rpm build
```
You can find the package in the `target/release/rpmbuild/RPMS/x86_64` directory. To install you can use the command below.

```bash
cd target/release/rpmbuild/RPMS/x86_64
sudo rpm -i target/release/rpmbuild/RPMS/x86_64/*.rpm
```

### Tarball
```bash
cargo build --release
tar -czvf cuur.tar.gz -C target/release cuur
```
If you look at your directory, you can see `cuur.tar.gz`.

### Arch
```bash
cargo aur
```
You can find the packages in `target/cargo-aur/`. If you look at that directory you can see the `PKGBUILD` file. To install you can use the command below.
```bash
cd target/cargo-aur
makepkg -si
```
# Examples

Install `vim` and `htop` in all of the distributions.
```toml
[sys]
works_on = ["all"]

[pkg]
install = ["vim", "htop"]
```

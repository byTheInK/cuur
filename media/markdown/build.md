# Building from Source

First, ensure you have the necessary packages installed. Make sure you have `curl` installed.

---

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

---

## Install Dependencies

### Debian-based systems:

```bash
sudo apt install cargo git rpm
```

### RHEL-based systems:

```bash
sudo dnf install cargo git rpm-build python-pipx
```

### Arch-based systems:

```bash
sudo pacman -S cargo git rpm python-pipx base-devel
```

---

## Clone the Repository

```bash
git clone https://github.com/byTheInK/cuur
cd cuur
```

---

## Install Building Tools

Pick the tool or tools you want to use for building packages.

```bash
cargo install cargo-deb # For Debian packages
cargo install cargo-rpm # For RPM packages
cargo install cargo-aur # For Arch Linux packages
cargo install cargo-bundle # For AppImage
```

---

## Build Packages

### Debian Package

```bash
cargo deb
```

You can find the package in the `target/debian/` directory. To install, use the command below:

```bash
cd target/debian/
sudo dpkg -i *.deb
```

---

### RPM Package

```bash
cargo rpm
```

You can find the package in the `target/release/rpmbuild/RPMS/x86_64/` directory. To install, use the command below:

```bash
cd target/release/rpmbuild/RPMS/x86_64/
sudo rpm -i *.rpm
```

---

### Tarball

```bash
cargo build --release
tar -czvf cuur.tar.gz -C target/release cuur
```

The `cuur.tar.gz` file will be created in your current directory.

---

### Arch Linux Package

```bash
cargo aur
```

You can find the package in the `target/cargo-aur/` directory. To install, use the command below:

```bash
cd target/cargo-aur
makepkg -si
```

---

### Notes:
1. **`cargo rpm build` vs `cargo rpm`**: The correct command for building RPM packages is `cargo rpm`, not `cargo rpm build`.
2. **`cargo deb build` vs `cargo deb`**: Similarly, the correct command for building Debian packages is `cargo deb`, not `cargo deb build`.
3. **Directory Paths**: Ensure the directory paths are correct when navigating to the built packages.
4. **Permissions**: You may need `sudo` to install the packages, depending on your system configuration.
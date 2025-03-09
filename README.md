# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers. This project makes distro-hopping a lot easier because you can write a configuration file to install the packages you want.

> [!WARNING]
> Cuur currently doesn't work in Windows unless you use Windows subsystem for Linux.

# Installation
You can get Cuur from [here](https://github.com/byTheInK/cuur/releases). Pick the correct package for your operating system or distribution. For Debian pick `.deb`, for Fedora `.rpm` and for Windows `.exe`.

## Building from source
You can check out [here](./media/markdown/build.md) to get the development version.

# Usage
You can use both `toml` and `yaml` for this file. I recommend you using a toml file but if you prefer, you can use a yaml file.

## Flags
There are currently four flags. These are: `--toml`, `--yaml`, `--json` and `--debug`. Toml flag gets added by default but if you want to use a yaml or a json file you have to use the yaml or json flag.

### Example:
```bash
cuur input.toml
```

```bash
cuur input.yml --yaml
```

## Basics
Let's make a file called `backup.toml`. Inside this file, we are going to put the packages we use.

```toml
[sys]
works_on = ["all"]

[pkg]
install = ["sudo", "vim", "nano", "htop", "chromium", "firefox"]
``` 
This file we made works in all of the distributions. In the `sys` tab we made a variable called `works_on`. We declared the string `all` in this variable so every distribution can use this file. In the `pkg` tab we declared a variable called `install`. We typed the names of the packages we want to install.

Now we can execute this with the command below.
```bash
cuur --toml ./backup.toml
```

We can also put distribution names into the works_on variable.
```toml
[sys]
works_on = ["Arch Linux", "Debian", "Ubuntu", "FreeBsd"]

[pkg]
install = ["htop"]
```

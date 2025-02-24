# cuur
Cuur is an alternative to Nix but the extra features is instead of the Nix package manager it uses the system package manager. You can also use AUR and AUR helpers.

# Installation
You can get Cuur from [here](https://github.com/byTheInK/cuur/releases). Pick the correct package for your operating system or distribution. For Debian pick `.deb`, for Fedora `.rpm` and for Windows `.exe'.

## Building from source
You can check out [here](./media/markdown/build.md) to get the development version.

# Usage
You can use both `toml` and `yaml` for this file. I recommend you using a roml file but if you prefer, you can use a yaml file.

## Flags
There are currently three flags. These are: `--toml`, `--yaml` and `--debug`. Toml flag gets added by default but if you want to use a yaml file you have to use the yaml flag.

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


use std::collections::HashMap;

pub fn get_package_manager_install(
    distro: &str,
) -> Option<(&'static str, &'static str, &'static str)> {
    let package_managers: HashMap<&str, (&str, &str, &str)> = HashMap::from([
        ("AIX", ("installp", "-a", "")),
        ("AlmaLinux", ("dnf", "install", "-y")),
        ("Alpaquita Linux", ("apk", "add", "--no-confirm")),
        ("Alpine Linux", ("apk", "add", "--no-confirm")),
        ("Amazon Linux AMI", ("yum", "install", "-y")),
        ("Android", ("pkg", "install", "-y")),
        ("Arch Linux", ("pacman", "-S", "--noconfirm")),
        ("Artix Linux", ("pacman", "-S", "--noconfirm")),
        ("CachyOS", ("pacman", "-S", "--noconfirm")),
        ("CentOS", ("dnf", "install", "-y")),
        ("Debian", ("apt", "install", "-y")),
        ("DragonFly BSD", ("pkg", "install", "-y")),
        ("Emscripten", ("emsdk", "install", "")),
        ("EndeavourOS", ("pacman", "-S", "--noconfirm")),
        ("Fedora", ("dnf", "install", "-y")),
        ("FreeBSD", ("pkg", "install", "-y")),
        ("Garuda Linux", ("pacman", "-S", "--noconfirm")),
        ("Gentoo Linux", ("emerge", "--ask n", "")),
        ("HardenedBSD", ("pkg", "install", "-y")),
        ("illumos", ("pkgin", "install", "-y")),
        ("Kali Linux", ("apt", "install", "-y")),
        ("Linux", ("varies", "", "")),
        ("Mabox", ("pacman", "-S", "--noconfirm")),
        ("Manjaro", ("pacman", "-S", "--noconfirm")),
        ("Mariner", ("tdnf", "install", "-y")),
        ("MidnightBSD", ("mport", "install", "")),
        ("Mint", ("apt", "install", "-y")),
        ("NetBSD", ("pkgin", "install", "-y")),
        ("NixOS", ("nix", "env -i", "")),
        ("Nobara Linux", ("dnf", "install", "-y")),
        ("OpenBSD", ("pkg_add", "", "")),
        ("OpenCloudOS", ("dnf", "install", "-y")),
        ("openEuler (EulerOS)", ("dnf", "install", "-y")),
        ("openSUSE", ("zypper", "install", "-y")),
        ("Oracle Linux", ("dnf", "install", "-y")),
        ("Pop!_OS", ("apt", "install", "-y")),
        ("Raspberry Pi OS", ("apt", "install", "-y")),
        ("Red Hat Linux", ("rpm", "-i", "")),
        ("Red Hat Enterprise Linux", ("dnf", "install", "-y")),
        ("Redox", ("pkg", "install", "-y")),
        ("Rocky Linux", ("dnf", "install", "-y")),
        ("Solus", ("eopkg", "install", "-y")),
        ("SUSE Linux Enterprise Server", ("zypper", "install", "-y")),
        ("Ubuntu", ("apt", "install", "-y")),
        ("Ultramarine Linux", ("dnf", "install", "-y")),
        ("Unknown", ("unknown", "", "")),
        ("Void Linux", ("xbps-install", "-S", "-y")),
    ]);

    package_managers.get(distro).copied()
}

pub fn get_package_manager_remove(
    distro: &str,
) -> Option<(&'static str, &'static str, &'static str)> {
    let package_managers: HashMap<&str, (&str, &str, &str)> = HashMap::from([
        ("AIX", ("installp", "-u", "")),
        ("AlmaLinux", ("dnf", "remove", "-y")),
        ("Alpaquita Linux", ("apk", "del", "--no-confirm")),
        ("Alpine Linux", ("apk", "del", "--no-confirm")),
        ("Amazon Linux AMI", ("yum", "remove", "-y")),
        ("Android", ("pkg", "uninstall", "-y")),
        ("Arch Linux", ("pacman", "-R", "--noconfirm")),
        ("Artix Linux", ("pacman", "-R", "--noconfirm")),
        ("CachyOS", ("pacman", "-R", "--noconfirm")),
        ("CentOS", ("dnf", "remove", "-y")),
        ("Debian", ("apt", "remove", "-y")),
        ("DragonFly BSD", ("pkg", "delete", "-y")),
        ("Emscripten", ("emsdk", "uninstall", "")),
        ("EndeavourOS", ("pacman", "-R", "--noconfirm")),
        ("Fedora", ("dnf", "remove", "-y")),
        ("FreeBSD", ("pkg", "delete", "-y")),
        ("Garuda Linux", ("pacman", "-R", "--noconfirm")),
        ("Gentoo Linux", ("emerge", "--depclean", "")),
        ("HardenedBSD", ("pkg", "delete", "-y")),
        ("illumos", ("pkgin", "remove", "-y")),
        ("Kali Linux", ("apt", "remove", "-y")),
        ("Linux", ("varies", "", "")),
        ("Mabox", ("pacman", "-R", "--noconfirm")),
        ("Manjaro", ("pacman", "-R", "--noconfirm")),
        ("Mariner", ("tdnf", "remove", "-y")),
        ("MidnightBSD", ("mport", "remove", "")),
        ("Mint", ("apt", "remove", "-y")),
        ("NetBSD", ("pkgin", "remove", "-y")),
        ("NixOS", ("nix-env", "-e", "")),
        ("Nobara Linux", ("dnf", "remove", "-y")),
        ("OpenBSD", ("pkg_delete", "", "")),
        ("OpenCloudOS", ("dnf", "remove", "-y")),
        ("openEuler (EulerOS)", ("dnf", "remove", "-y")),
        ("openSUSE", ("zypper", "remove", "-y")),
        ("Oracle Linux", ("dnf", "remove", "-y")),
        ("Pop!_OS", ("apt", "remove", "-y")),
        ("Raspberry Pi OS", ("apt", "remove", "-y")),
        ("Red Hat Linux", ("rpm", "-e", "")),
        ("Red Hat Enterprise Linux", ("dnf", "remove", "-y")),
        ("Redox", ("pkg", "remove", "-y")),
        ("Rocky Linux", ("dnf", "remove", "-y")),
        ("Solus", ("eopkg", "remove", "-y")),
        ("SUSE Linux Enterprise Server", ("zypper", "remove", "-y")),
        ("Ubuntu", ("apt", "remove", "-y")),
        ("Ultramarine Linux", ("dnf", "remove", "-y")),
        ("Unknown", ("unknown", "", "")),
        ("Void Linux", ("xbps-remove", "-R", "-y")),
    ]);

    package_managers.get(distro).copied()
}

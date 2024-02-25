# Color picker

A color picker tool developed in Rust.

## Dependencies

### xdotool

This project relies on the "xdotool" library for tracking the mouse position. Make sure to install it before running 
your program.

#### Arch Linux

```bash
pacman -S xdotool
```

#### Debian-based

```bash
apt install libxdo-dev
```

#### Fedora

```bash
dnf install libX11-devel libxdo-devel
```

#### Gentoo

```bash
emerge -a xdotool
```
# Speedtest

A morden cli speedtest tool, **zero dependency**, with graph integration and metrics export.

### Quick install:
```bash
curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/master/scripts/install.sh | sh
```
---
### Demo
![Speedtest Demo](docs/static/Screenshot%202026-04-04%20at%2017.48.44.png)

---
### Installation
Quick install (autodetect best method):
`curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/master/scripts/install.sh | sh`

Or, pick your preference:

| Platform | Method |
|----------|--------|
| macOS | [brew](#brew) |
| Bazzite/Bluefin/Aurora Linux | [brew](#brew) |
| Windows | [winget](#windows-winget) |
| Arch Linux | [AUR](#arch-linux-aur) |
| Debian/Ubuntu | [.deb](#debianubuntu-deb) |
| Fedora/RHEL | [.rpm](#fedorarhelopensuse-rpm), [Terra](https://terra.fyralabs.com/) |
| FreeBSD | [ports / pkg](https://www.speedtestports.org/editors/speedtest) |
| Linux (any distro) | [AppImage](#appimage), [Flatpak](#flatpak) |
| All platforms | [Pre-built binaries](#pre-built-binaries) |
| npm | [npm / npx](#npm) |
| Rust users (Fast) | [cargo-binstall](#using-cargo-binstall) |
| Rust users | [crates.io](#from-cratesio) |
| Nix | [Nix flakes](#nix-flakes) |
| Developers | [From source](#from-source) |

### Brew

On macOS and some linux distros (Bazzite/Bluefin/Aurora):

> **Note:** On macOS, see [macOS Terminal Tips](https://getspeedtest.dev/docs/configuration/keyboard#macos-terminal-tips) for recommended terminal configuration.

```bash
brew tap bacteriafield/speedtest
brew install speedtest-tool
```

### Windows (winget)

```bash
winget install speedtest-tool
```

Alternatively, Windows users can use [npm](#npm).

### Arch Linux ([AUR](https://aur.archlinux.org/packages/speedtest-tool-bin))

**Binary package (recommended, faster install):**

```bash
git clone https://aur.archlinux.org/speedtest-tool-bin.git
cd speedtest-tool-bin
makepkg --syncdeps --install
```

**Build from source:**

```bash
git clone https://aur.archlinux.org/speedtest-tool.git
cd speedtest-tool
makepkg --syncdeps --install
```

**Using an AUR helper (such as `yay` or `paru`):**

```bash
# Binary package (recommended, faster install)
yay -S speedtest-tool-bin

# Or build from source
yay -S speedtest-tool
```

### Debian/Ubuntu (.deb)

Download and install the latest release:

```bash
curl -sL $(curl -s https://api.github.com/repos/bacteriafield/speedtest/releases/latest | grep "browser_download_url.*_$(dpkg --print-architecture)\.deb" | cut -d '"' -f 4) -o speedtest-tool.deb && sudo dpkg -i speedtest-tool.deb
```

Or download the `.deb` file manually from the [releases page](https://github.com/bacteriafield/speedtest/releases).

### Fedora/RHEL/openSUSE (.rpm)

Download and install the latest release:

```bash
curl -sL $(curl -s https://api.github.com/repos/bacteriafield/speedtest/releases/latest | grep "browser_download_url.*\.$(uname -m)\.rpm" | cut -d '"' -f 4) -o speedtest-tool.rpm && sudo rpm -U speedtest-tool.rpm
```

Or download the `.rpm` file manually from the [releases page](https://github.com/bacteriafield/speedtest/releases).

### AppImage

Download the `.AppImage` file from the [releases page](https://github.com/bacteriafield/speedtest/releases) and run:

```bash
chmod +x speedtest-tool-VERSION-x86_64.AppImage
./speedtest-tool-VERSION-x86_64.AppImage
```

**For faster startup** (recommended): Extract the AppImage instead of running it directly. This avoids the FUSE mount overhead on each launch (~10x faster):

```bash
./speedtest-tool-VERSION-x86_64.AppImage --appimage-extract
mkdir -p ~/.local/share/speedtest-tool ~/.local/bin
mv squashfs-root/* ~/.local/share/speedtest-tool/
ln -sf ~/.local/share/speedtest-tool/usr/bin/speedtest ~/.local/bin/speedtest
```

Ensure `~/.local/bin` is in your PATH. Available for x86_64 and aarch64 architectures.

### Flatpak

Download the `.flatpak` bundle from the [releases page](https://github.com/bacteriafield/speedtest/releases) and install:

```bash
flatpak install --user speedtest-tool-VERSION-x86_64.flatpak
flatpak run io.github.sinelaw.speedtest
```

See [flatpak/README.md](flatpak/README.md) for building from source.

### Pre-built binaries

Download the latest release for your platform from the [releases page](https://github.com/bacteriafield/speedtest/releases).

### npm

```bash
npm install -g @speedtest-tool/speedtest-tool
```

Or try it without installing:

```bash
npx @speedtest-tool/speedtest-tool
```

### Using cargo-binstall

To install the binary directly without compiling (much faster than crates.io):

First, install cargo-binstall if you haven't already

```bash
cargo install cargo-binstall
```

Then install speedtest

```bash
cargo binstall speedtest-tool
```

### Nix flakes

Run without installing:
```bash
nix run github:bacteriafield/speedtest
```

Or install to your profile:
```bash
nix profile add github:bacteriafield/speedtest
```

### From crates.io

```bash
cargo install --locked speedtest-tool
```

### From source

```bash
git clone https://github.com/bacteriafield/speedtest.git
cd speedtest
cargo build --release
./target/release/speedtest <flag>
```

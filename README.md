# Speedtest

Uma ferramenta moderna de teste de velocidade via linha de comando, sem dependências, com integração de gráficos e exportação de métricas.

### Quick install:
```bash
curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/main/scripts/install.sh | sh
```
---
### Demo
![Speedtest Demo](docs/static/Screenshot%202026-04-04%20at%2017.48.44.png)

---
### Installation
Quick install (autodetect best method):
`curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/main/scripts/install.sh | sh`

Or, pick your preference:

| Platform | Method |
|----------|--------|
| macOS | [brew](#brew) |
| Bazzite/Bluefin/Aurora Linux | [brew](#brew) |
| Debian/Ubuntu | [.deb](#debianubuntu-deb) |
| Fedora/RHEL | [.rpm](#fedorarhelopensuse-rpm), [Terra](https://terra.fyralabs.com/) |
| FreeBSD | [ports / pkg](https://www.speedtestports.org/editors/speedtest) |
| Linux (any distro) | [AppImage](#appimage), [Flatpak](#flatpak) |
| All platforms | [Pre-built binaries](#pre-built-binaries) |
| Rust users (Fast) | [cargo-binstall](#using-cargo-binstall) |
| Rust users | [crates.io](#from-cratesio) |
| Developers | [From source](#from-source) |

### Brew

On macOS and some linux distros (Bazzite/Bluefin/Aurora):

> **Note:** On macOS, see [macOS Terminal Tips](https://getspeedtest.dev/docs/configuration/keyboard#macos-terminal-tips) for recommended terminal configuration.

```bash
brew tap bacteriafield/speedtest
brew install speedtest-tool
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

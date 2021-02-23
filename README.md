# DEPRECATED

No longer maintained. Please consider replacing with [Pastel](https://github.com/sharkdp/pastel).

---

# cocore

A CLI tool converts color representation such as HSL colors and RGB colors.

## Installation

Linux

```console
$ version=v0.0.0
$ curl -L -o /usr/local/bin/cocore https://github.com/KoharaKazuya/cocore/releases/download/$version/cocore-x86_64-unknown-linux-gnu
$ chmod +x /usr/local/bin/cocore
```

macOS (Homebrew)

```console
$ brew install KoharaKazuya/misc/cocore
```

## Usage

```console
$ cocore --help
cocore 0.1.0
KoharaKazuya
converts color representation such as HSL colors and RGB colors

USAGE:
    cocore [OPTIONS] [expression]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --to <representation>    color representation cocore converts into [default: hex]  [possible values: hex, rgb,
                                 hsl]

ARGS:
    <expression>...
$ cocore --to hsl 'rgb(0, 100, 200)'
hsl(210, 100%, 39%)
```

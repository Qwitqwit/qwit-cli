# qwit-cli

## Links

[crates.io](https://crates.io/crates/qwit)

## [Documentation](DOC.md)

## Install the cli

### Cargo

```bash
    cargo install qwit
```

### Docker

```bash
    docker pull ghcr.io/qwitqwit/qwit-cli:v1.3.1
```

### Code with Cargo

```bash
    git clone https://github.com/Qwitqwit/qwit-cli
    cd qwit-cli
    cargo build --release
    cargo install --path .
```

## Usage

### Native

```bash
    qwit --help
```

### Docker

To run a command on a file you need to mount the file first
You may abstract this into another command or into docker compose
```bash
    docker run --rm -v ./testfiles/tmp/test.csv:/testfiles/tmp/test.csv -t ghcr.io/qwitqwit/qwit-cli:v1.3.1 show --source testfiles/tmp/test.csv
```


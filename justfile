set dotenv-load := true

alias v := verify
alias r := run

bt := '0'
log := "warn"

@_list:
    just --list --unsorted

run:
    cargo run

install:
    cargo build --release
    cargo install --path .

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------

# Watch the source files and run `just verify` when source changes
watch:
    cargo watch -- just verify

# Run the tests
test:
    cargo hack test --feature-powerset

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo hack clippy --feature-powerset --all-targets --workspace

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
# you may need to install perl IPC cmd "yum install perl-IPC-Cmd" and install the perl module "cpan install File::Remote"
install-dev-tools:
    rustup install stable
    rustup override set stable
    cargo install cargo-hack cargo-watch cargo-deny
    cargo install cargo-release

clean:
    rm -rf target
    rm -f Cargo.lock
    rm -rf node_modules

fmt:
    cargo fmt

pub:
    cargo publish


release *args: verify
    test $GITHUB_TOKEN
    test $CARGO_REGISTRY_TOKEN
    cargo release {{args}}
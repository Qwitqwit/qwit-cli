set dotenv-load := true

alias v := verify
alias r := run

image_name := "ghcr.io/optravis-llc/qwit-cli"

bt := '0'
log := "warn"

@_list:
    just --list --unsorted

run:
    cargo run

install:
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

# latest doc file
create-doc: install
    qwit markdown > DOC.md


release *args: verify
    test $GITHUB_TOKEN
    test $CARGO_REGISTRY_TOKEN
    cargo release {{args}}



# example how to run it, you need to build it first obv.
run-docker:
    cd distributions && just build-docker
    docker run --rm -v ./testfiles/tmp/test.csv:/testfiles/tmp/test.csv -t {{image_name}}:latest show --source testfiles/tmp/test.csv
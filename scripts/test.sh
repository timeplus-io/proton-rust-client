# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# https://nexte.st/book/installing-from-source.html
# cargo install cargo-nextest --locked

command cargo test doc

cargo nextest run
# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo clean

#command rm -rf gen/
#command rm -rf target/
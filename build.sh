# CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release --color always 2>&1 | less -r
CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release --color always 2>&1
RUST_BACKTRACE=1 cargo test --release

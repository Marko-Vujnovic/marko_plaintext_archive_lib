# TODO: . <(ssh-agent); ssh-add $HOME/.ssh/markoVujnovicKey.privateKey
NP_RUNTIME=proot GIT_COMMITTER_DATE="8 June 1994 01:33:08" GIT_AUTHOR_DATE="8 June 1994 01:33:08" GIT_COMMITTER_NAME="Marko-Vujnovic" GIT_AUTHOR_NAME="Marko-Vujnovic" CARGO_NET_GIT_FETCH_WITH_CLI=true NIXPKGS_ALLOW_UNFREE=1 HOME=$HOME/nixHome nix-portable nix-shell -E "let oxalica_overlay = import (builtins.fetchTarball \"https://github.com/oxalica/rust-overlay/archive/master.tar.gz\"); nixpkgs = import (builtins.fetchTarball \"https://github.com/NixOS/nixpkgs/archive/a7ecde854aee5c4c7cd6177f54a99d2c1ff28a31.tar.gz\") { overlays = [ oxalica_overlay ]; }; in with nixpkgs; stdenv.mkDerivation { name = \"rust-env\"; buildInputs = [ jetbrains.idea-community (rust-bin.stable.latest.default.override { extensions = [ \"rust-src\" ]; })  pkg-config openssl linuxPackages.perf curl gnupg ]; RUST_BACKTRACE = 1; }" --option sandbox false --command "idea-community .; return"

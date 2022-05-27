# Must be run from root of the repo:
# yew $ ./ci/write-optimisation-flags.sh

# this goes in [unstable] section
cat >> .cargo/config.toml << EOF
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]
EOF
cat >> Cargo.toml << EOF
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
opt-level = "z"
EOF

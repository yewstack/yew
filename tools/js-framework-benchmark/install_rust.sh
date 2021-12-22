curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /root/rustup.sh
sh /root/rustup.sh -y 
source /root/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
rustup toolchain install nightly
rustup override set nightly
cat /root/.cargo/env >> /etc/bashrc

#!/usr/bin/env bash
cargo build --release
cp ./target/release/hex hex
echo "Moving file into /usr/bin/ (will require sudo perms)"
echo "Alternatively exit out and move the `hex` file "
echo "created by this script to your desired location."
sudo mv hex /usr/bin/
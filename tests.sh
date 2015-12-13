#!/bin/sh
cargo run res/1Color >> decktests
cargo run res/2ColorBalanced >> decktests
cargo run res/2ColorUnbalanced >> decktests
cargo run res/3ColorBalanced >> decktests
cargo run res/3ColorUnbalanced >> decktests
cargo run res/4ColorBalanced >> decktests
cargo run res/4ColorUnbalanced >> decktests
cargo run res/5ColorBalanced >> decktests
cargo run res/5ColorUnbalanced >> decktests

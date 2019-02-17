# Reproducer for Rustc issue [58548](https://github.com/rust-lang/rust/issues/58548)

## Show the expected behaviour in 1.31.0
0. rustup toolchain add 1.31.0
1. rustup override set 1.31.0
2. yarn && yarn build
3. yarn serve
4. open localhost:8080
5. console shows Error: "no longer broken"

## Show the broken behaviour in 1.32.0
0. rustup toolchain add 1.32.0
1. rustup override set 1.32.0
2. yarn && yarn build
3. yarn serve
4. open localhost:8080
5. console shows Info: "generated code is broken"


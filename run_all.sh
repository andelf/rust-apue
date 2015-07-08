#!/bin/sh +x


cargo run --example ls1 ..

echo "HELLO WORLD" | cargo run --example mycat

echo hello | cargo run --example getcputc

cargo run --example hello

echo date | cargo run --example shell1

cargo run --example testerror

cargo run --example uidgid

cargo run --example seek

cargo run --example hole

cargo run --example fileflags 1

cargo run --example setfl

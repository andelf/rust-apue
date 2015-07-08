#!/bin/sh +x


cargo run --example ls1 ..

cargo run --example mycat < README.md

echo hello | cargo run --example getcputc

cargo run --example hello

echo date | cargo run --example shell1

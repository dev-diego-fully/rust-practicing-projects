mkdir ./scheduler
cargo build --release --features mlua/$1
mv ./target/release/libcore.so ./scheduler/core.so
cp ./stubs/scheduler.lua ./scheduler/init.lua
cargo clean
# rdial
server port dial, rust impl. coroutine support.

## quick start

```rust
cargo run --release
```

find execute target, run with args.

```rust
cd target/release
./rdial --start-port 80 --end-port 2000 --hostname 127.0.0.1 --timeout 200
```

```shell
// output
Opt { hostname: "127.0.0.1", start_port: 80, end_port: 2000, timeout: 200 }
opened ports: [1080, 1086, 1087]
```

## notice

os fd limit may made rdial unexpected result. please check os limit.

```shell
ulimit -n
```

```shell
// output
4864
```

while run with rdial `--start-port 1000` and `--end-port 9000`, 9000 - 1000 = 8000, and 8000 > 4864, since dial result maybe unexpected.
so make sure your os limit bigger than your args.

**↑ PLEASE HELP ME RESOLVE THIS CASE THANKS ↑**

email: nickchenyx @ gmail dot com

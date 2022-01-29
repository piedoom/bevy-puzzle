
## Building for web

```rs
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir target\web --target web target\wasm32-unknown-unknown\release\puzzle.wasm
```

Then you can use

```
basic-http-server
```

buglist:

- Placing right before a timer is done may result in the timer going off before the event to place the tile has been executed causing unexpected behavior
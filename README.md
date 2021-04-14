# maze-generator-two

A WebAssembly library written in Rust, which generates mazes, and also generates
vertex buffers and indices to be used in a WebGL context to render out as basic
maze.

## Docs

### Build Instructions

To build the WebAssembly module, you need
[Rust](https://www.rust-lang.org/tools/install) and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

Once you have those installed, to build the module, run
```sh
wasm-pack build
```

To build without the `console_error_panic_hook` feature (adds better console
logging on panic, but increases build size quite a lot), remove the default
features,
```sh
wasm-pack build -- --no-default-features
```

To build with `wee_alloc` feature (decreases build size a bit, but a bit slower
memory allocation),
```sh
wasm-pack build -- --features wee_alloc
```


You can also combine both the options as you like.

### Usage

Once built, you'll be left with a WebAssembly module in `./pkg`. To include the
module in you npm or Yarn, add it to the `dependencies` section as such:
```json
{
  "dependencies": {
    "maze-generator-two": "portal:<path/to/pkg>"
  }
}
```

The `maze-generator-two` module exports three objects:
* `MazeAlgo`: An enum representing the available maze generation algorithm.
* `SquareMaze`: A class representing a square maze.
* `SqMzBuffer`: A buffer generator which generates the required buffers to
                render the maze using a WebGL context.

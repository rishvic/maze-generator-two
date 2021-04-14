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
              Available options right now are:
  1. `MazeAlgo.Dfs`
  2. `MazeAlgo.Kruskal`
* `SquareMaze`: A class representing a square maze.
  1. `SquareMaze::from_seed(width: usize, height: usize, algo: MazeAlgo, seed_p1: u32, seed_p2: u32) -> SquareMaze`
    - Generates a new maze of the given width, height, using the provided Maze
      Algorithm and the two seed numbers.
* `SqMzBuffer`: A buffer generator which generates the required buffers to
                render the maze using a WebGL context.
  1. `SqMzBuffer::new(mz: SquareMaze, r: f32) -> SqMzBuffer`
    - Generates new square maze vertex and index buffers, using the provided
      SquareMaze as reference, and `r` representing the ratio of thickness of
      maze wall to maze path.
  2. `SqMzBuffer.get_vertices(&self) -> *const f32, SqMzBuffer.get_vertices_count(&self) -> usize`
    - Returns the vertex array and its length.
  3. `SqMzBuffer.get_indices(&self) -> *const f32, SqMzBuffer.get_indices_count(&self) -> usize`
    - Returns the indices array and its length.

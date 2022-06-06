# Snake

A Snake Game built with Rust and WebAssembly

<p align="center">
  <img src="https://github.com/omerwyo/snakegame/blob/main/video.gif?raw=true" alt="Gameplay"/>
</p>

## Building

Make sure you have [Rust](https://www.rust-lang.org) and [wasm-pack](https://rustwasm.github.io/wasm-pack/) installed. 

To install `wasm-pack` on *nix systems:

```bash
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or follow the instructions [here](https://rustwasm.github.io/wasm-pack/installer/#).

To build this project, run:

```
$ wasm-pack build --target web
```

To run this project, you need a static file server. You can install `serve` with npm:

```
$ npm install serve -g
```

Now, start your static file server and open `index.html`:

```
$ serve
```

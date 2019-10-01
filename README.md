# Toryn

## Running binaries

Make sure that you have installed stable Rust toolchain, if you don't have it you
can install it with [`rustup`](https://rustup.rs/): 

```
$ rustup install stable
$ rustup default stable
```

After that, you should have all the necessary to run binaries inside `src/bin` folder
for that choose one to run (e.g. `draw_mid_line`) 

```
$ cargo run --bin draw_mid_line
```


## Binaries description

- `draw_inc_line`: Draws some lines using the incremental method.
- `draw_mid_line`: Draws some lines using the middle point method.
- `draw_shape2d:` Draw a shape by giving at least 3 points, it's possible to change the line draw method too.
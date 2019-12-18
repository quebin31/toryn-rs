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

## Demos 

`draw_bezier` demo:
![Bezier](demo/bezier.gif)


## Binaries 

- `draw_inc_line`: Draws some lines using the incremental method.
- `draw_mid_line`: Draws some lines using the middle point method.
- `draw_shape2d`: Draw a shape by giving at least 3 points, it's possible to change the line draw method too.
- `draw_bezier:` Draw a bezier curve from clicking points on screen.
- `camera_proj`: Play with the camera and perspective.
    - `W` move camera forward.
    - `S` move camera backward.
    - `D` move camera to the right.
    - `S` move camera to the left.
    - `+` increase fov.
    - `-` decrease fov.
    - `x` rotates over x.
    - `y` rotates over y.
    - `z` rotates over z.
    - `Up` move object forward.
    - `Down` move object backward.
    - `Left` move object to the left.
    - `Right` move object to the right.
    - `Space` move object up.
    - `LShift` move object down.
    - Move the mouse to yaw and pitch the camera.
    - `Tab` changes perspective to orthographic projection, and viceversa (currently broken).
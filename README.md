# Holed Cuboid

Create a cuboid with holes


## Usage

```
$ cargo run -- -h
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/holed-cuboid -h`
Generate a cubiod mesh with holes and write it to an STL file

Usage: holed-cuboid [OPTIONS]

Options:
  -w, --width <WIDTH>                          width of the panel [default: 10.0]
  -l, --length <LENGTH>                        length of the panel [default: 100.0]
  -H, --height <HEIGHT>                        height of the panel [default: 5.0]
  -d, --diameter <DIAMETER>                    hole diameter, zero for no hole [default: 0]
  -s, --support-thickness <SUPPORT_THICKNESS>  hole support thickness [default: 0]
  -h, --help                                   Print help
  -V, --version                                Print version
  ```

## Build, run, install

```
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
wink@3900x 25-08-15T17:58:30.445Z:~/csgrs-holed-cubiod
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/holed-cuboid`
 arg.version: 0.1.0
outside_dimenstions: [[10.0, 100.0, 5.0]]
Writing STL file: holed-cuboid_w-10.00_l-100.00_h-5.00.stl
wink@3900x 25-08-15T17:58:47.390Z:~/csgrs-holed-cubiod
$ cargo install --path .
  Installing holed-cuboid v0.1.0 (/home/wink/csgrs-holed-cubiod)
  Installing /home/wink/.cargo/bin/holed-cuboid
   Installed package `holed-cuboid v0.1.0 (/home/wink/csgrs-holed-cubiod)` (executable `holed-cuboid`)
```

## Results

The default output file is `cuboid_w-10.00_l-100.00_h-5.00.stl`

This can be visualized as a 3D model using `f3d cuboid_w-10.00_l-100.00_h-5.00.stl`
and converted to an image using `f3d cuboid_w-10.00_l-100.00_h-5.00.stl --output cuboid_w-10.00_l-100.00_h-5.00.png`

#![cuboid](cuboid_w-10.00_l-100.00_h-5.00.png)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

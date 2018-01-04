# slippy_map_tilenames

A Rust crates that converts lon/lat coordinates to slippy map tile format. See [this article](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames) by *wiki.openstreetmap.org*

## Example

```rust
extern crate slippy_map_tilenames as smt;

fn main() {
    let t2l = smt::tile2lonlat(4376, 2932, 13); // (12.3046875, 45.460130637921)
    let l2t = smt::lonlat2tile(14.016667, 42.683333, 13); // (4414, 3019)    
    println!("Tile (4376, 2932) at zoom 13: {:?}", t2l);
    println!("lon 14.016667 E, lat 42.683333 N, at zoom 13: {:?}", l2t);
}
```

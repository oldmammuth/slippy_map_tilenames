//! # Slippy Map Tilenames
//! 
//! The `slippy_map_tilenames` crate provides functions to convert lon/lat coordinates to slippy map tile format.
//! See this [article](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames) by `wiki.openstreetmap.org`
//! 
//! Some other functions to help in dealing with the slippy map format are provided as well.
//!
//! # Breaking Changes
//! 
//! This crate is designed for retrocompatibility; i.e., to provide functions that consistently 
//! have the same signature, no matter which version of the crate you will use.
//! 
//! # Warning
//!
//! All the functions provided by this crate **do not** check the validity of the data in input.
//! However, since they are based on equations, some of them still give in output a result in presence of invalid input; 
//! this result is indeed meaningles.
//! 
//! The care of checking on the validity of the data in input is left to the user of this crate.
//! 
//! For further reference, see the sections *Unexpected Behavior* related to each function.

use std::f64;
use std::f64::consts::PI;

/// Convert lon/lat coordinates to a slippy map tile, at a given zoom.
///
/// # Arguments
///
/// * `lon`  - longitude coordinate (W-E), in degrees
/// * `lat`  - latitude  coordinate (N-S), in degrees
/// * `zoom` - zoomlevel of the resulting tile
///
/// # Examples
///
/// ```
/// extern crate slippy_map_tilenames as smt;
///
/// fn main() {
///     let res = smt::lonlat2tile(14.016667, 42.683333, 13); // (4414, 3019)
///     println!("lon 14.016667 E, lat 42.683333 N, at zoom 13: {:?}", res); 
/// }
/// ```
///
/// # Unexpected Behavior
///
/// The conversion relies on an equation, which **does not** check on the *validity* of the data in imput.
/// For this reason, passing non valid coordinates still gets in output a result, albeit *meaningless* .
///
/// Therefore, care should be taken as to the input of the function in order to pass it valid tiles.
/// Example:
///
/// ```
/// extern crate slippy_map_tilenames as smt;
/// 
/// assert_eq!(smt::lonlat2tile(123456.789, 123456.789, 0), (343, 0));
/// ```
///
/// Notice how at zoomlevel 0 there is really just a tile, (0, 0);
/// Moreover, (123456.789, 123456.789) is not a valid coordinate; 
/// However, the function still calculates a meaningless result.
///
/// Cf. the following:
///
/// ```
/// extern crate slippy_map_tilenames as smt;
/// use std::f64;
/// 
/// assert_eq!(smt::lonlat2tile(f64::INFINITY, f64::NEG_INFINITY, 0), (0, 0));
/// ```
pub fn lonlat2tile(lon: f64, lat: f64, zoom: u8) -> (u32, u32) {
    let lat_rad = lat.to_radians();
    let zz: f64 = 2f64.powf(zoom as f64) as f64;
    let x: u32 = ( (lon + 180f64) / 360f64 * zz ) as u32;
    let y: u32 = (( 1f64 - ( lat_rad.tan() + ( 1f64 / lat_rad.cos() ) ).ln() / PI )  / 2f64 * zz ) as u32;
    (x, y)
}

/// Convert slippy map tile to lon/lat coordinates, at a given zoom.
///
/// # Arguments
///
/// * `x`  - X tile coordinate
/// * `y`  - Y tile coordinate
/// * `zoom` - zoomlevel of the tile
///
/// # Examples
///
/// ```
/// extern crate slippy_map_tilenames as smt;
///
/// fn main() {
///     let res = smt::tile2lonlat(4376, 2932, 13); // (12.3046875, 45.460130637921)
///     println!("Tile (4376, 2932) at zoom 13: {:?}", res);
/// }
/// ```
///
/// # Unexpected Behavior
///
/// The conversion relies on an equation, which **does not** check on the *validity* of the data in imput.
/// For this reason, passing non valid tiles for the relative zoomlevel still gets in output a result, albeit *meaningless*.
///
/// Therefore, care should be taken as to the input of the function in order to pass it valid tiles.
///
/// Example:
///
/// ```
/// extern crate slippy_map_tilenames as smt;
/// 
/// assert_eq!(smt::tile2lonlat(4376, 2932, 0), (1575180.0, -90.0));
/// ```
///
/// Notice how at zoomlevel 0 there is really just a tile, (0, 0), yet the function still calculates a meaningless result.
pub fn tile2lonlat(x: u32, y: u32, zoom: u8) -> (f64, f64) {
    let zz: f64 = 2f64.powf(zoom as f64) as f64;
    let lon: f64 = x as f64 / zz * 360f64 - 180f64;
    let lat: f64 = ( ( PI * (1f64 - 2f64 * y as f64 / zz) ).sinh() ).atan().to_degrees();
    (lon, lat)
}

/// Zooms in starting from the given tile
///
/// The `zoom in` function, given a tile `t:(x, y)` corresponding to the zoomlevel `z`, 
/// returns the 4 tiles onto which the tile `t` is split out zooming to the next zoomlevel `z + 1`.
///
/// The actual zoomlevel is of no consequence to the function, since the tile `t` will be split
/// out into 4 tiles when zooming in, as per the definitions of a *web slippy map*.
///
/// For more info, cf the relevant [article](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Subtiles) in `wiki.openstreetmap.org`
///
/// # Arguments
///
/// * `x`  - X current tile coordinate
/// * `y`  - Y current tile coordinate
///
/// Notice how the zoomlevel is unimportant, and therefore not needed as argument, since many 
/// tile's X and Y coordinates are shared among many different zoomlevels.
/// 
/// # Output
///
/// This functions gets in output 4 tiles referenced as follows:
/// 
/// ```((x1, y1), (x2, y1), (x1, y2), (x2, y2))```
///
/// Grafically:
///
/// ```text
/// +--------+--------+
/// | x1, y1 | x2, y1 |
/// +--------+--------+
/// | x1, y2 | x2, y2 |
/// +--------+--------+
/// ```
///
/// # Examples
///
/// ```
/// extern crate slippy_map_tilenames as smt;
///
/// fn main() {
///     let (x, y) = smt::zoom_out(5,7); // (2,3)
///     println!("t: (5,7), z: 3; zooming out to z: 2 => ({}, {})", x, y);
/// }
/// ```
///
/// # Unexpected Behavior
///
/// This function does not behave unexpectedly. 
/// However care should be taken in providing the tile's coordinates in input, 
/// since many tile's X and Y coordinates are shared among many different zoomlevels.
pub fn zoom_in(x: u32, y: u32) -> ( (u32, u32), (u32, u32), (u32, u32), (u32, u32) ) {
    let x2 = 2 * x;
    let y2 = 2 * y;
    ( (x2, y2), (x2 + 1, y2 ), (x2, y2 + 1), (x2 + 1, y2 + 1) )
}

/// Zooms out starting from the given tile
///
/// The `zoom out` function, given a tile `t:(x, y)` corresponding to the zoomlevel `z`, 
/// returns the tile `t_out` into which the tile `t` is merged when zooming to the previous 
/// zoomlevel `z - 1`.
///
/// The actual zoomlevel is of no consequence to the function, since when zooming in the tile 
/// `t_out` will be split out into 4 tiles, one of which is in fact the present tile `t`, 
/// as per the definitions of a *web slippy map*.
///
/// For more info, cf. the relevant [article](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Subtiles) in `wiki.openstreetmap.org`
///
/// Refer also to the `Output` section of the `zoom_in` function.
///
/// # Arguments
///
/// * `x`  - X current tile coordinate
/// * `y`  - Y current tile coordinate
///
/// Notice how the zoomlevel is unimportant, and therefore not needed as argument, since many tile's X and Y coordinates 
/// are shared among many different zoomlevels.
/// 
/// # Examples
///
/// ```
/// extern crate slippy_map_tilenames as smt;
///
/// fn main() {
///     let ((a1, b1), (c1, d1), (a2, b2), (c2, d2)) = smt::zoom_in(1,1);
///     println!("+------+------+");
///     println!("| {}, {} | {}, {} |", a1, b1, c1, d1 );
///     println!("+------+------+");
///     println!("| {}, {} | {}, {} |", a2, b2, c2, d2 );
///     println!("+------+------+");
/// }
/// ```
///
/// # Unexpected Behavior
///
/// This function does not behave unexpectedly. 
/// However care should be taken in providing the tile's coordinates in input, 
/// since many tile's X and Y coordinates are shared among many different zoomlevels.
///
/// Cf. the following limit case, where zooming out from the tile (0, 0)
/// only obtains the same tile (0, 0):
///
/// ```
/// extern crate slippy_map_tilenames as smt;
/// 
/// assert_eq!(smt::zoom_out(0, 0), (0, 0));
/// ```
pub fn zoom_out(x: u32, y: u32) -> (u32, u32) {
    ( x / 2, y / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::mpsc;
    
    #[test]
    fn test_t2l_basic() {
        assert_eq!(tile2lonlat(4376, 2932, 13), (12.3046875, 45.460130637921) );
        assert_eq!(tile2lonlat(0, 0, 0), (-180.0, 85.0511287798066));
        assert_eq!(tile2lonlat(4376, 2932, 0), (1575180.0, -90.0));
    }
    #[test]
    fn test_l2t_basic() {
        assert_eq!(lonlat2tile(12.3046875, 45.460130637921, 13), (4376, 2932) );
        assert_eq!(lonlat2tile(0.0, 0.0, 0), (0, 0));
        assert_eq!(lonlat2tile(123456.789, 123456.789, 0), (343, 0));
        assert_eq!(lonlat2tile(f64::INFINITY, f64::NEG_INFINITY, 0), (0, 0));
    }
    #[test]
    fn zoom_basics(){
        assert_eq!(zoom_in(1,1), ((2, 2), (3, 2), (2, 3), (3, 3)) );
        assert_eq!(zoom_out(5,7), (2,3));
        assert_eq!(zoom_out(0, 0), (0, 0));
    }
    #[test]
    fn test_multithreading() {
        let (tx1, rx) = mpsc::channel();
        let tx2 = mpsc::Sender::clone(&tx1);
        let tx3 = mpsc::Sender::clone(&tx1);
        let tx4 = mpsc::Sender::clone(&tx1);

        let _ = thread::spawn(move || {
            let (ln, lt) = tile2lonlat(4376, 2932, 13);
            tx1.send( (1, (format!("{:.3}", ln), format!("{:.3}", lt))) ).unwrap();
        });
        
        let _ = thread::spawn(move || {
            let (x, y) = lonlat2tile(12.3046875, 45.460130637921, 13);
            tx2.send( (2, (x.to_string(), y.to_string())) ).unwrap();
        });

        let _ = thread::spawn(move || {
            let ((_, _), (x, y), (_, _), (_, _)) = zoom_in(1, 1);
            tx3.send( (3, (x.to_string(), y.to_string())) ).unwrap();
        });

        let _ = thread::spawn(move || {
            let (x, y) = zoom_out(5, 7);
            tx4.send( (4, (x.to_string(), y.to_string())) ).unwrap();
        });

        for received in rx {
            match received.0 {
                1 => assert_eq!( received.1, ("12.305".into(), "45.460".into()) ),
                2 => assert_eq!( received.1, ("4376".into(), "2932".into()) ),
                3 => assert_eq!( received.1, ("3".into(), "2".into()) ),
                4 => assert_eq!( received.1, ("2".into(), "3".into()) ),
                _ => unreachable!()
            }
        }
    }
}

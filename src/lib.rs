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
pub fn tile2lonlat(x: u32, y: u32, zoom: u8) -> (f64, f64) {
    let zz: f64 = 2f64.powf(zoom as f64) as f64;
    let lon: f64 = x as f64 / zz * 360f64 - 180f64;
    let lat: f64 = ( ( PI * (1f64 - 2f64 * y as f64 / zz) ).sinh() ).atan().to_degrees();
    (lon, lat)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_t2l() {
        assert_eq!(tile2lonlat(4376, 2932, 13), (12.3046875, 45.460130637921) );
    }
    #[test]
    fn test_l2t() {
        assert_eq!(lonlat2tile(12.3046875, 45.460130637921, 13), (4376, 2932) );
    }
}

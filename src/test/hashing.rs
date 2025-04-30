use crate::tiles::Tile;
use crate::utils::point_cover;

#[test]
fn test_tile_hashing() {
    let cases = [
        (Tile::new(0, 0, 0), 0_u64),
        (Tile::new(1, 0, 1), 33_u64),
        (Tile::new(0, 1, 1), 129_u64),
        (Tile::new(0, 0, 2), 2_u64),
        (Tile::new(13, 13, 13), 6816173_u64),
        (Tile::new(46, 3584, 12), 939525580_u64),
        (Tile::new(123, 321, 25), 689342254969_u64),
        (Tile::new(8108, 14336, 14), 15032645006_u64),
    ];

    for (tile, hash) in cases.iter() {
        // Tile to hash
        assert_eq!(tile.to_hash(), *hash);
        // Hash to tile
        assert_eq!(Tile::from_hash(*hash), *tile);
    }
}

#[test]
fn test_point_hashing() {
    let cases = [
        ((46.152, -52.222), 10_u8, 44978282_u64),
        ((46.152, -52.222), 22_u8, 755128617831862_u64),
        ((46.152, -52.222), 26_u8, 193312953173859226_u64),
    ];

    for (coords, res, hash) in cases.iter() {
        assert_eq!(point_cover(coords.1, coords.0, *res), *hash);
    }
}

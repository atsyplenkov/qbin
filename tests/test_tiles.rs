use quadbin::tiles::*;
use quadbin::types::Tile;

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
        assert_eq!(to_tile_hash(tile), *hash);
        // Hash to tile
        assert_eq!(from_tile_hash(*hash), *tile);
    }
}

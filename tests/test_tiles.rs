use quadbin::tiles::*;
use quadbin::types::Tile;

#[test]
fn test_tile_to_hash() {
    let cases = [
        (Tile::new(0, 0, 0), 0_u64),
        (Tile::new(1, 0, 1), 33_u64),
        (Tile::new(0, 1, 1), 129_u64),
        (Tile::new(0, 0, 2), 2_u64),
        (Tile::new(46, 3584, 12), 939525580_u64),
        (Tile::new(8108, 14336, 14), 15032645006_u64),
    ];

    for (tile, expected) in cases.iter() {
        assert_eq!(to_tile_hash(tile), *expected);
    }
}

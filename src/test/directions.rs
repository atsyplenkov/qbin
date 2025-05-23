use crate::directions::*;

#[test]
fn test_valid_direction() {
    let dirs = [0, 1, 2, 3];

    for val in dirs.iter() {
        assert_eq!(u8::from(Direction::new_unchecked(*val)), { *val });
        assert_eq!(u64::from(Direction::new_unchecked(*val)), *val as u64);
        assert_eq!(usize::from(Direction::new_unchecked(*val)), *val as usize);
    }

    assert_eq!(u8::from(Direction::Up), 0_u8);
    assert_eq!(u8::from(Direction::Right), 1_u8);
    assert_eq!(u8::from(Direction::Down), 3_u8);
    assert_eq!(u8::from(Direction::Left), 2_u8);

    assert_eq!(Direction::try_from(0).ok(), Some(Direction::Up));
    assert_eq!(Direction::try_from(1).ok(), Some(Direction::Right));
    assert_eq!(Direction::try_from(2).ok(), Some(Direction::Left));
    assert_eq!(Direction::try_from(3).ok(), Some(Direction::Down))
}

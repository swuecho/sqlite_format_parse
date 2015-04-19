// convert 2 byte u8 to a u16
pub fn get2byte(p: &[u8]) -> u16 {
  assert_eq!(p.len(), 2);
  (p[0] as u16) << 8u8  | p[1] as u16
}

// convert 4 byte u8 to a u32
pub fn get4byte(p: &[u8]) -> u32 {
    // how to case each u8 in array?
    assert_eq!(p.len(), 4);
    (p[0] as u32) <<24 | (p[1] as u32) <<16 | (p[2] as u32) << 8 | p[3] as u32
}


#[test]
fn get2byte_works() {
    assert_eq!(get2byte(&[4,0]), 1024u16);
}

#[test]
fn get4byte_works() {
    assert_eq!(get4byte(&[4,0,0,0]), 67108864u32);
}



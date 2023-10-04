const fn all_pos_mask() -> [u64; 64] {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = 1 << i;
        i += 1;
    }
    table
}
pub const POSES: [u64; 64] = all_pos_mask();

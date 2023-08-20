//参考:https://qiita.com/watergreat31/items/09f114799319c7ba19dc
pub const BOTTOM_MASK: u64 = 0xFFFFFFFFFFFFFF00;
pub const TOP_MASK: u64 = 0x00FFFFFFFFFFFFFF;
pub const LEFT_MASK: u64 = 0x7F7F7F7F7F7F7F7F;
pub const RIGHT_MASK: u64 = 0xFEFEFEFEFEFEFEFE;
pub const HORIZONTAL_MASK: u64 = RIGHT_MASK & LEFT_MASK;
pub const VERTICAL_MASK: u64 = TOP_MASK & BOTTOM_MASK;
pub const DIAGONAL_MASK: u64 = HORIZONTAL_MASK & VERTICAL_MASK;
pub const TOP_LEFT_MASK: u64 = TOP_MASK & LEFT_MASK;
pub const TOP_RIGHT_MASK: u64 = TOP_MASK & RIGHT_MASK;
pub const BOTTOM_LEFT_MASK: u64 = BOTTOM_MASK & LEFT_MASK;
pub const BOTTOM_RIGHT_MASK: u64 = BOTTOM_MASK & RIGHT_MASK;

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

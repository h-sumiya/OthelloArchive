//https://qiita.com/watergreat31/items/09f114799319c7ba19dc

/*
左右用
_ x x _
_ x x _
_ x x _
_ x x _
*/
pub const HORIZONTAL: u64 = 0x7E7E7E7E7E7E7E7E;


/*
上下用
_ _ _ _
x x x x
x x x x
_ _ _ _
*/
pub const VERTICAL: u64 = 0x00FFFFFFFFFFFF00;

/*
斜め用
_ _ _ _
_ x x _
_ x x _
_ _ _ _
*/
pub const DIAGONAL: u64 = 0x007E7E7E7E7E7E00;
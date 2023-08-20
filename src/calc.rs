use crate::base::Board; //python:del

const fn calc_fs(i: u8, j: u8) -> i8 {
    let mut socre = 0;
    if i & j != 0 {
        return 0;
    }
    if i | j == 255 {
        socre = i.count_ones() as i8 - j.count_ones() as i8;
    } else {
        socre += i.leading_ones() as i8;
        socre += i.trailing_ones() as i8;
        socre -= j.leading_ones() as i8;
        socre -= j.trailing_ones() as i8;
    }
    socre *= 2;
    if i & 1 != 0 {
        socre -= 1;
    }
    if i & 128 != 0 {
        socre -= 1;
    }
    if j & 1 != 0 {
        socre += 1;
    }
    if j & 128 != 0 {
        socre += 1;
    }
    socre
}

const fn calc_fses() -> [[i8; 256]; 256] {
    let mut table = [[0i8; 256]; 256];
    let mut i = 0u8;
    loop {
        let mut j = 0u8;
        loop {
            let socre = calc_fs(i, j);
            table[i as usize][j as usize] = socre;
            if j == 255 {
                break;
            }
            j += 1;
        }
        if i == 255 {
            break;
        }
        i += 1;
    }
    table
}

const fn calc_point() -> [[isize; 256]; 8] {
    let mut table = [[0; 256]; 8];
    let data = [
        [45, -11, 4, -1, -1, 4, -11, 45],
        [-11, -16, -1, -3, -3, -1, -16, -11],
        [4, -1, 2, -1, -1, 2, -1, 4],
        [-1, -3, -1, 0, 0, -1, -3, -1],
        [-1, -3, -1, 0, 0, -1, -3, -1],
        [4, -1, 2, -1, -1, 2, -1, 4],
        [-11, -16, -1, -3, -3, -1, -16, -11],
        [45, -11, 4, -1, -1, 4, -11, 45],
    ];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 256 {
            let mut point = 0;
            let mut k = 0;
            while k < 8 {
                if j & (1 << k) != 0 {
                    point += data[i][k];
                }
                k += 1;
            }
            table[i][j] = point;
            j += 1;
        }
        i += 1;
    }
    table
}

const POINT_TABLE: [[isize; 256]; 8] = calc_point();
pub const FS_TABLE: [[i8; 256]; 256] = calc_fses();

//[参考]https://qiita.com/ysuzuk81/items/9ee9d0a295471bb6d1ef
fn get_top_edge(board: u64) -> usize {
    ((board & 0xFF00000000000000) >> 56) as usize
}

fn get_bottom_edge(board: u64) -> usize {
    (board & 0x00000000000000FF) as usize
}

fn get_left_edge(board: u64) -> usize {
    ((board & 0x8080808080808080) * 0x0002040810204081 >> 56) as usize
}

fn get_right_edge(board: u64) -> usize {
    ((board & 0x0101010101010101) * 0x0102040810204080 >> 56) as usize
}

impl Board {
    pub fn count(&self) -> isize {
        self.me.count_ones() as isize + self.opp.count_ones() as isize
    }

    pub fn remain(&self) -> usize {
        64 - self.count() as usize
    }

    pub fn kn(&self) -> isize {
        self.me.count_ones() as isize - self.opp.count_ones() as isize
    }

    pub fn cn(&self) -> isize {
        let me = self.legal_moves().count_ones() as isize;
        let opp = self.opp_legal_moves().count_ones() as isize;
        me - opp
    }

    pub fn bp(&self) -> isize {
        let mut points = 0;
        unsafe {
            for (i, table) in POINT_TABLE.iter().rev().enumerate() {
                let me_index = ((self.me >> (i * 8)) & 0x00000000000000FF) as usize;
                points += *table.get_unchecked(me_index);
                let opp_index = ((self.opp >> (i * 8)) & 0x00000000000000FF) as usize;
                points -= *table.get_unchecked(opp_index);
            }
        }
        points
    }

    pub fn fs(&self) -> isize {
        let mut score = 0i8;
        unsafe {
            score += *FS_TABLE
                .get_unchecked(get_top_edge(self.me))
                .get_unchecked(get_top_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_bottom_edge(self.me))
                .get_unchecked(get_bottom_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_left_edge(self.me))
                .get_unchecked(get_left_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_right_edge(self.me))
                .get_unchecked(get_right_edge(self.opp));
        }
        score as isize
    }

    pub fn score(&self) -> isize {
        self.cn() * 20 + self.bp() * 2 + self.fs() * 75
    }
}

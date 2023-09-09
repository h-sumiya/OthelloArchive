use crate::base::Board;
use crate::cell_name::*;
use crate::pack::*;
use crate::pos::Pos;
use std::io::{Cursor, Read, Seek};

const EVAL: &[u8] = include_bytes!("./eval.dat");

fn get_index(board: &Board, cellcoords: &[usize]) -> usize {
    let mut fi = 0;
    for cellcoord in cellcoords {
        let pos = Pos::from_py(*cellcoord);
        if board.me & pos.0 != 0 {
            fi = fi * 3;
        } else if board.opp & pos.0 != 0 {
            fi = fi * 3 + 1;
        } else {
            fi = fi * 3 + 2;
        }
    }
    fi
}

const PACKED_WEIGHT_N_SIZE: [usize; 13] = [
    10206, 29889, 29646, 29646, 3321, 3321, 3321, 3321, 1134, 378, 135, 45, 1,
];
const PACK_DEF_MAP: [usize; 13] = [0, 1, 2, 2, 3, 3, 3, 3, 4, 5, 6, 7, 8];

pub struct EvalData {
    pub packed_weight: Vec<Vec<Vec<i16>>>,
    pub index_map: Vec<Vec<usize>>,
}

impl EvalData {
    pub fn load() -> Self {
        let mut cur = Cursor::new(EVAL);
        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf).unwrap();
        let edax = u32::from_le_bytes(buf);
        cur.read_exact(&mut buf).unwrap();
        let eval = u32::from_le_bytes(buf);
        if edax != 0x45444158 || eval != 0x4556414c {
            panic!("eval.dat is invalid");
        }
        cur.seek(std::io::SeekFrom::Start(28)).unwrap();
        let mut buf = [0u8; 2];
        let mut paked_weight = vec![];
        for _ in 0..=60 {
            let mut ply_arr = vec![];
            for size in &PACKED_WEIGHT_N_SIZE {
                let mut arr = vec![];
                for _ in 0..*size {
                    cur.read_exact(&mut buf).unwrap();
                    let ply = i16::from_le_bytes(buf);
                    arr.push(ply);
                }
                ply_arr.push(arr);
            }
            paked_weight.push(ply_arr);
        }

        let mut index_map = vec![];
        for pack_def in PACKED_DEF {
            let mut n = 0;
            let mut arr = vec![];
            for j in 0..pack_def.total {
                let k = pack_def.symmetry(j);
                if k < j {
                    arr.push(arr[k]);
                } else {
                    arr.push(n);
                    n += 1;
                }
            }
            index_map.push(arr);
        }
        Self {
            packed_weight: paked_weight,
            index_map: index_map,
        }
    }

    pub fn evaluate(&self, board: &Board) -> f64 {
        let mut value_list = vec![];
        let mut score_total = 0;

        let mut array = vec![];
        for cellcoords in FEATURE0.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[0];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][0][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE1.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[1];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][1][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE2.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[2];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][2][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE3.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[3];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][3][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE4.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[4];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][4][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE5.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[5];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][5][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE6.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[6];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][6][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE7.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[7];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][7][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE8.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[8];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][8][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE9.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[9];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][9][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE10.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[10];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][10][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        for cellcoords in FEATURE11.iter() {
            let index = get_index(board, cellcoords);
            let map_index = PACK_DEF_MAP[11];
            let map = &self.index_map[map_index];
            let value = self.packed_weight[board.turn()][11][map[index]];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        let mut array = vec![];
        {
            let value = self.packed_weight[board.turn()][12][0];
            array.push(value);
            score_total += value as isize;
        }
        value_list.push(array);

        Self::score(score_total)
    }

    fn score(total: isize) -> f64 {
        let val = total as f64 / 128.0;
        if val < -64.0 {
            -64.0
        } else if val > 64.0 {
            64.0
        } else {
            val
        }
    }
}

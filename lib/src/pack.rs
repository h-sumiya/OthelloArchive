pub struct PackData {
    pub total: usize,
    pub cells: usize,
    pub id: usize,
}

impl PackData {
    pub fn symmetry(&self, l: usize) -> usize {
        match self.id {
            0 => {
                let mut symmetry = 0;
                symmetry += ((l / 6561) % 3) * 6561;
                symmetry += ((l / 729) % 3) * 2187;
                symmetry += ((l / 2187) % 3) * 729;
                symmetry += ((l / 243) % 3) * 243;
                symmetry += ((l / 27) % 3) * 81;
                symmetry += ((l / 81) % 3) * 27;
                symmetry += ((l / 3) % 3) * 9;
                symmetry += ((l / 9) % 3) * 3;
                symmetry += l % 3;
                return symmetry;
            }
            1 => {
                let mut symmetry = 0;
                symmetry += (l / 19683) % 3;
                symmetry += ((l / 6561) % 3) * 3;
                symmetry += ((l / 2187) % 3) * 9;
                symmetry += ((l / 729) % 3) * 27;
                symmetry += ((l / 243) % 3) * 243;
                symmetry += ((l / 81) % 3) * 81;
                symmetry += ((l / 27) % 3) * 729;
                symmetry += ((l / 9) % 3) * 2187;
                symmetry += ((l / 3) % 3) * 6561;
                symmetry += (l % 3) * 19683;
                return symmetry;
            }
            2 => {
                let mut symmetry = 0;
                symmetry += (l / 19683) % 3;
                symmetry += ((l / 6561) % 3) * 3;
                symmetry += ((l / 2187) % 3) * 9;
                symmetry += ((l / 729) % 3) * 27;
                symmetry += ((l / 243) % 3) * 81;
                symmetry += ((l / 81) % 3) * 243;
                symmetry += ((l / 27) % 3) * 729;
                symmetry += ((l / 9) % 3) * 2187;
                symmetry += ((l / 3) % 3) * 6561;
                symmetry += (l % 3) * 19683;
                return symmetry;
            }
            3 => {
                let mut symmetry = 0;
                symmetry += (l / 2187) % 3;
                symmetry += ((l / 729) % 3) * 3;
                symmetry += ((l / 243) % 3) * 9;
                symmetry += ((l / 81) % 3) * 27;
                symmetry += ((l / 27) % 3) * 81;
                symmetry += ((l / 9) % 3) * 243;
                symmetry += ((l / 3) % 3) * 729;
                symmetry += (l % 3) * 2187;
                return symmetry;
            }
            4 => {
                let mut symmetry = 0;
                symmetry += (l / 729) % 3;
                symmetry += ((l / 243) % 3) * 3;
                symmetry += ((l / 81) % 3) * 9;
                symmetry += ((l / 27) % 3) * 27;
                symmetry += ((l / 9) % 3) * 81;
                symmetry += ((l / 3) % 3) * 243;
                symmetry += (l % 3) * 729;
                return symmetry;
            }
            5 => {
                let mut symmetry = 0;
                symmetry += (l / 243) % 3;
                symmetry += ((l / 81) % 3) * 3;
                symmetry += ((l / 27) % 3) * 9;
                symmetry += ((l / 9) % 3) * 27;
                symmetry += ((l / 3) % 3) * 81;
                symmetry += (l % 3) * 243;
                return symmetry;
            }
            6 => {
                let mut symmetry = 0;
                symmetry += (l / 81) % 3;
                symmetry += ((l / 27) % 3) * 3;
                symmetry += ((l / 9) % 3) * 9;
                symmetry += ((l / 3) % 3) * 27;
                symmetry += (l % 3) * 81;
                return symmetry;
            }
            7 => {
                let mut symmetry = 0;
                symmetry += (l / 27) % 3;
                symmetry += ((l / 9) % 3) * 3;
                symmetry += ((l / 3) % 3) * 9;
                symmetry += (l % 3) * 27;
                return symmetry;
            }
            8 => 0,
            _ => l,
        }
    }
}

pub const PACKED_DEF: [PackData; 9] = [
    PackData {
        total: 19683,
        cells: 9,
        id: 0,
    },
    PackData {
        total: 59049,
        cells: 10,
        id: 1,
    },
    PackData {
        total: 59049,
        cells: 10,
        id: 2,
    },
    PackData {
        total: 6561,
        cells: 8,
        id: 3,
    },
    PackData {
        total: 2187,
        cells: 7,
        id: 4,
    },
    PackData {
        total: 729,
        cells: 6,
        id: 5,
    },
    PackData {
        total: 243,
        cells: 5,
        id: 6,
    },
    PackData {
        total: 81,
        cells: 4,
        id: 7,
    },
    PackData {
        total: 1,
        cells: 0,
        id: 8,
    },
];

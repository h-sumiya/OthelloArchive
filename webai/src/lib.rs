use wasm_bindgen::prelude::*;
mod ai;
mod base;
mod book;
mod calc;
mod mask;
mod pos;
mod score;


#[wasm_bindgen]
pub fn varsion() -> String {
    "0.1.0".to_string()
}

#[wasm_bindgen]
pub fn ai(data: Vec<usize>, ai: usize) -> usize {
    let b = base::Board::new(data, ai);
    let pos = b.ai();
    pos.js_pos()
}

#[wasm_bindgen]
pub fn legal_moves(data: Vec<usize>) -> Vec<usize> {
    let b = base::Board::new(data, 1);
    let mut res = vec![];
    let moves = b.legal_moves();
    for i in 0..64 {
        let pos = 1 << i;
        if moves & pos != 0 {
            res.push(i);
        }
    }
    res
}

#[wasm_bindgen]
pub fn put(data: Vec<usize>, pos: usize, me: usize, opp: usize) -> Vec<usize> {
    let b = base::Board::new(data, me);
    let pos = pos::Pos::from_js(pos);
    let b = b.put(pos);
    b.js_data(opp, me)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut data = vec![0; 64];
        data[27] = 1;
        data[28] = 2;
        data[35] = 2;
        data[36] = 1;
        let pos = super::ai(data.clone(), 1);
        let b = super::base::Board::new(data, 1);
        assert_eq!(!b.legal_moves() & (1 << pos), 0);
    }
}

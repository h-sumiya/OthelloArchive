
const moves = [
    [1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [1, -1], [-1, 1], [-1, -1]
];

function inrange(x, y) {
    if (!(0 <= x && x < 8)) {
        return false
    } else if (!(0 <= y && y < 8)) {
        return false
    } else {
        return true
    }
}
export function filp(date, pos, color) {
    let filps = [];
    let oppcolor = 3 - color;
    let x = pos % 8;
    let y = Math.floor(pos / 8);
    if (date[pos] !== 0) {
        throw 'すでに石が置かれていました';
    }
    for (let i = 0; i < 8; i++) {
        let move_x = moves[i][0];
        let move_y = moves[i][1];
        let tmpfilps = [];
        let next_x = x + move_x;
        let next_y = y + move_y;
        while (inrange(next_x, next_y) && date[next_y * 8 + next_x] == oppcolor) {
            tmpfilps.push(next_y * 8 + next_x);
            next_x += move_x;
            next_y += move_y;

        } if (inrange(next_x, next_y) && date[next_y * 8 + next_x] == color) {
            filps = [...filps, ...tmpfilps]
        }

    }
    if (filps.length == 0) {
        throw '置けません';
    }
    let res = [...date];
    for (let i = 0; i < filps.length; i++) {
        res[filps[i]] = color
    }
    res[pos] = color;
    return res;
}

export function check(date, pos, color) {
    let oppcolor = 3 - color;
    let x = pos % 8;
    let y = Math.floor(pos / 8);
    if (date[pos] !== 0) {
        return false;
    }
    for (let i = 0; i < 8; i++) {
        let move_x = moves[i][0];
        let move_y = moves[i][1];
        let next_x = x + move_x;
        let next_y = y + move_y;
        let flag = false;
        while (inrange(next_x, next_y) && date[next_y * 8 + next_x] == oppcolor) {
            flag = true;
            next_x += move_x;
            next_y += move_y;

        } if (inrange(next_x, next_y) && date[next_y * 8 + next_x] == color && flag) {
            return true;
        }

    }
    return false
}

export function legal_moves(date, color) {
    let moves = [];
    for (let i = 0; i < 64; i++) {
        if (check(date, i, color)) {
            moves.push(i);
        }
    }
    return moves
}

export function have_legal_moves(date, color) {
    for (let i = 0; i < 64; i++) {
        if (check(date, i, color)) {
            return true;
        }
    }
    return false;

}

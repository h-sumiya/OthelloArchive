const moves = [
    [1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [1, -1], [-1, 1], [-1, -1]
];

function in_range(x, y) {
    if (!(0 <= x && x < 8)) {
        return false
    } else if (!(0 <= y && y < 8)) {
        return false
    } else {
        return true
    }
}

export function flip(data, pos, color) {
    let flips = [];
    let oppcolor = 3 - color;
    let x = pos % 8;
    let y = Math.floor(pos / 8);
    if (data[pos] !== 0) {
        throw 'すでに石が置かれていました';
    }
    for (let i = 0; i < 8; i++) {
        let move_x = moves[i][0];
        let move_y = moves[i][1];
        let tmpflips = [];
        let next_x = x + move_x;
        let next_y = y + move_y;
        while (in_range(next_x, next_y) && data[next_y * 8 + next_x] == oppcolor) {
            tmpflips.push(next_y * 8 + next_x);
            next_x += move_x;
            next_y += move_y;

        } if (in_range(next_x, next_y) && data[next_y * 8 + next_x] == color) {
            flips = [...flips, ...tmpflips]
        }

    }
    if (flips.length == 0) {
        throw '置けません';
    }
    let res = [...data];
    for (let i = 0; i < flips.length; i++) {
        res[flips[i]] = color
    }
    res[pos] = color;
    return res;
}

export function check(data, pos, color) {
    let oppcolor = 3 - color;
    let x = pos % 8;
    let y = Math.floor(pos / 8);
    if (data[pos] !== 0) {
        return false;
    }
    for (let i = 0; i < 8; i++) {
        let move_x = moves[i][0];
        let move_y = moves[i][1];
        let next_x = x + move_x;
        let next_y = y + move_y;
        let flag = false;
        while (in_range(next_x, next_y) && data[next_y * 8 + next_x] == oppcolor) {
            flag = true;
            next_x += move_x;
            next_y += move_y;

        } if (in_range(next_x, next_y) && data[next_y * 8 + next_x] == color && flag) {
            return true;
        }

    }
    return false
}

export function legal_moves(data, color) {
    let moves = [];
    for (let i = 0; i < 64; i++) {
        if (check(data, i, color)) {
            moves.push(i);
        }
    }
    return moves
}

export function have_legal_moves(data, color) {
    for (let i = 0; i < 64; i++) {
        if (check(data, i, color)) {
            return true;
        }
    }
    return false;
}

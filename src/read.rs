use std::io; //python:del

pub fn read_usize(buf: &mut String) -> usize {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
    buf.trim().parse().unwrap()
}

pub fn read_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
    buf.pop();
}

pub fn skip_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
}

pub fn skip_lines(buf: &mut String, n: usize) {
    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(buf).unwrap();
    }
}

pub fn read_id(buf: &mut String) -> char {
    io::stdin().read_line(buf).unwrap();
    let id = buf.trim().chars().next().unwrap();
    skip_line(buf);
    id
}

pub fn skip_after(buf: &mut String) {
    let num = read_usize(buf);
    skip_lines(buf, num);
}

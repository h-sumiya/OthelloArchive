pub const A1: usize = 0;
pub const B1: usize = 1;
pub const C1: usize = 2;
pub const D1: usize = 3;
pub const E1: usize = 4;
pub const F1: usize = 5;
pub const G1: usize = 6;
pub const H1: usize = 7;
pub const A2: usize = 8;
pub const B2: usize = 9;
pub const C2: usize = 10;
pub const D2: usize = 11;
pub const E2: usize = 12;
pub const F2: usize = 13;
pub const G2: usize = 14;
pub const H2: usize = 15;
pub const A3: usize = 16;
pub const B3: usize = 17;
pub const C3: usize = 18;
pub const D3: usize = 19;
pub const E3: usize = 20;
pub const F3: usize = 21;
pub const G3: usize = 22;
pub const H3: usize = 23;
pub const A4: usize = 24;
pub const B4: usize = 25;
pub const C4: usize = 26;
pub const D4: usize = 27;
pub const E4: usize = 28;
pub const F4: usize = 29;
pub const G4: usize = 30;
pub const H4: usize = 31;
pub const A5: usize = 32;
pub const B5: usize = 33;
pub const C5: usize = 34;
pub const D5: usize = 35;
pub const E5: usize = 36;
pub const F5: usize = 37;
pub const G5: usize = 38;
pub const H5: usize = 39;
pub const A6: usize = 40;
pub const B6: usize = 41;
pub const C6: usize = 42;
pub const D6: usize = 43;
pub const E6: usize = 44;
pub const F6: usize = 45;
pub const G6: usize = 46;
pub const H6: usize = 47;
pub const A7: usize = 48;
pub const B7: usize = 49;
pub const C7: usize = 50;
pub const D7: usize = 51;
pub const E7: usize = 52;
pub const F7: usize = 53;
pub const G7: usize = 54;
pub const H7: usize = 55;
pub const A8: usize = 56;
pub const B8: usize = 57;
pub const C8: usize = 58;
pub const D8: usize = 59;
pub const E8: usize = 60;
pub const F8: usize = 61;
pub const G8: usize = 62;
pub const H8: usize = 63;

pub const FEATURE0: [[usize; 9]; 4] = [
    [A1, B1, A2, B2, C1, A3, C2, B3, C3],
    [H1, G1, H2, G2, F1, H3, F2, G3, F3],
    [A8, A7, B8, B7, A6, C8, B6, C7, C6],
    [H8, H7, G8, G7, H6, F8, G6, F7, F6],
];

pub const FEATURE1: [[usize; 10]; 4] = [
    [A5, A4, A3, A2, A1, B2, B1, C1, D1, E1],
    [H5, H4, H3, H2, H1, G2, G1, F1, E1, D1],
    [A4, A5, A6, A7, A8, B7, B8, C8, D8, E8],
    [H4, H5, H6, H7, H8, G7, G8, F8, E8, D8],
];

pub const FEATURE2: [[usize; 10]; 4] = [
    [B2, A1, B1, C1, D1, E1, F1, G1, H1, G2],
    [B7, A8, B8, C8, D8, E8, F8, G8, H8, G7],
    [B2, A1, A2, A3, A4, A5, A6, A7, A8, B7],
    [G2, H1, H2, H3, H4, H5, H6, H7, H8, G7],
];

pub const FEATURE3: [[usize; 10]; 4] = [
    [A1, C1, D1, C2, D2, E2, F2, E1, F1, H1],
    [A8, C8, D8, C7, D7, E7, F7, E8, F8, H8],
    [A1, A3, A4, B3, B4, B5, B6, A5, A6, A8],
    [H1, H3, H4, G3, G4, G5, G6, H5, H6, H8],
];
pub const FEATURE4: [[usize; 8]; 4] = [
    [A2, B2, C2, D2, E2, F2, G2, H2],
    [A7, B7, C7, D7, E7, F7, G7, H7],
    [B1, B2, B3, B4, B5, B6, B7, B8],
    [G1, G2, G3, G4, G5, G6, G7, G8],
];
pub const FEATURE5: [[usize; 8]; 4] = [
    [A3, B3, C3, D3, E3, F3, G3, H3],
    [A6, B6, C6, D6, E6, F6, G6, H6],
    [C1, C2, C3, C4, C5, C6, C7, C8],
    [F1, F2, F3, F4, F5, F6, F7, F8],
];
pub const FEATURE6: [[usize; 8]; 4] = [
    [A4, B4, C4, D4, E4, F4, G4, H4],
    [A5, B5, C5, D5, E5, F5, G5, H5],
    [D1, D2, D3, D4, D5, D6, D7, D8],
    [E1, E2, E3, E4, E5, E6, E7, E8],
];
pub const FEATURE7: [[usize; 8]; 2] = [
    [A1, B2, C3, D4, E5, F6, G7, H8],
    [A8, B7, C6, D5, E4, F3, G2, H1],
];
pub const FEATURE8: [[usize; 7]; 4] = [
    [B1, C2, D3, E4, F5, G6, H7],
    [H2, G3, F4, E5, D6, C7, B8],
    [A2, B3, C4, D5, E6, F7, G8],
    [G1, F2, E3, D4, C5, B6, A7],
];
pub const FEATURE9: [[usize; 6]; 4] = [
    [C1, D2, E3, F4, G5, H6],
    [A3, B4, C5, D6, E7, F8],
    [F1, E2, D3, C4, B5, A6],
    [H3, G4, F5, E6, D7, C8],
];
pub const FEATURE10: [[usize; 5]; 4] = [
    [D1, E2, F3, G4, H5],
    [A4, B5, C6, D7, E8],
    [E1, D2, C3, B4, A5],
    [H4, G5, F6, E7, D8],
];
pub const FEATURE11: [[usize; 4]; 4] = [
    [D1, C2, B3, A4],
    [A5, B6, C7, D8],
    [E1, F2, G3, H4],
    [H5, G6, F7, E8],
];

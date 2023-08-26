from dataclasses import dataclass

head = """macro_rules! load_data {
    ($name:ident,$size:expr,$val:expr) => {
        const $name: [u8; $size] = {
            const fn f() -> [u8; $size] {
                let data = $val.as_bytes();
                let mut v = [0; $size];
                let mut i = 0;
                while i < $size {
                    let val = (data[i * 2] - 0xd0) * 64 + (data[i * 2 + 1] - 0x80);
                    v[i] = val;
                    i += 1;
                }
                v
            }
            f()
        };
    };
}
"""

body = """load_data!({}, {}, "{}");"""


@dataclass
class RustData:
    name: str
    value: bytes

    def encode(self):
        text = ""
        for b in self.value:
            c1 = 0xd0 + b // 64
            c2 = 0x80 + b % 64
            text += bytes([c1, c2]).decode("utf-8")
        return body.format(
            self.name,
            len(self.value),
            text
        )


class RustBuilder:
    def __init__(self) -> None:
        self.datas: list[RustData] = []

    def add(self, name: str, value: bytes):
        self.datas.append(RustData(name, value))

    def build(self):
        text = ""
        for data in self.datas:
            text += data.encode() + "\n"
        return head + text

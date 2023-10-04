from dataclasses import dataclass
import pyperclip


def make_tabel():
    table = {}
    count = 0
    for i in [0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xee, 0xef]:
        for j in range(0x80, 0xbf + 1):
            for k in range(0x80, 0xbf + 1):
                table[count] = bytes([i, j, k]).decode("utf-8")
                count += 1
    for j in range(0x80, 0xbf + 1):
        for k in range(0x80, 0xbf + 1):
            for l in range(0x80, 0xbf + 1):
                table[count] = bytes([0xf1, j, k, l]).decode("utf-8")
                if count == 0xFFFF:
                    return table
                count += 1


HEADER = """const {}:{} = """
BODY = """include_data!({}, "{}");"""
TABLE = make_tabel()


@dataclass
class Header:
    name: str
    type: str
    pub: bool = False

    def encode(self):
        data = HEADER.format(self.name, self.type)
        if self.pub:
            data = "pub " + data
        return data


@dataclass
class RustData:
    value: bytes
    header: Header = None

    def encode(self):
        size = len(self.value)
        value = list(self.value) + [0] * (size % 2)
        res = ""
        for i in range(0, size, 2):
            res += TABLE[value[i] + value[i + 1] * 256]
        if self.header:
            return self.header.encode() + BODY.format(size, res)
        return BODY.format(size, res)


class RustBuilder:
    def __init__(self) -> None:
        self.datas: list[RustData] = []

    def add(self, value: bytes, header: Header = None):
        self.datas.append(RustData(value, header))

    def build(self, copy: bool = True):
        text = ""
        for data in self.datas:
            text += data.encode() + "\n"
        if copy:
            pyperclip.copy(text)
        print("build success len:", len(text))
        return text


if __name__ == "__main__":
    builder = RustBuilder()
    data = []
    for i in range(256):
        for j in range(256):
            data.append(i)
            data.append(j)
    builder.add(bytes(data))
    builder.add(bytes([0]))
    builder.add(bytes([0, 192]))
    builder.build()

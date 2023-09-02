from data import Pos

EDGE = [
    Pos.load(p)
    for p in [
        "a1", "b1", "c1", "d1",
        "e1", "f1", "g1", "h1",
    ]
]


EDGE2 = [
    Pos.load(p)
    for p in [
        "a1", "c1", "d1", "e1", "f1", "h1",
        "c2", "d2", "e2", "f2"
    ]
]

CORNER = [
    Pos.load(p)
    for p in [
        "a1", "b1", "c1", "d1",
        "a2", "b2", "c2",
        "a3", "b3",
        "a4"
    ]
]

CROSS = [
    Pos.load(p)
    for p in [
        "a1", "b2", "c3", "d4",
        "e5", "f6", "g7", "h8",
    ]
]

EDGES = [[int(e.rotate(i)) for e in EDGE] for i in range(4)]
EDGES2 = [[int(e.rotate(i)) for e in EDGE2] for i in range(4)]
CORNERS = [[int(c.rotate(i)) for c in CORNER] for i in range(4)]
CROSSES = [[int(c.rotate(i)) for c in CROSS] for i in (0, 3)]


if __name__ == "__main__":
    from pprint import pprint
    from tools import *
    import struct
    print("======EDGE=====")
    pprint(EDGES)
    print("======EDGE2=====")
    pprint(EDGES2)
    print("======CORNER=====")
    pprint(CORNERS)
    print("======CROSS=====")
    pprint(CROSSES)

    FULL = EDGES2 + CORNERS + CROSSES
    # [[[u16;10];8];8]
    data = b""
    for i in range(8):
        for j in range(8):
            id = i * 8 + j
            for e in FULL:
                if id in e:
                    data += struct.pack("H", 3 ** e.index(id))
                else:
                    data += struct.pack("H", 0)
    print(len(data))

    rsb = RustBuilder()
    rsb.add(data, Header(
        "INDEXES",
        f"[[[u16;10];8];8]"
    ))
    rsb.build()

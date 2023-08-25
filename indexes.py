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
    print("======EDGE=====")
    pprint(EDGES)
    print("======EDGE2=====")
    pprint(EDGES2)
    print("======CORNER=====")
    pprint(CORNERS)
    print("======CROSS=====")
    pprint(CROSSES)

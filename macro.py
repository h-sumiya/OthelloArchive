# calc_score_all!(EDGE_TABLE, EDGE_MODEL, 3000, 8, A, 0, 1000, B, 1000, 1000, C, 2000, 1000);

name = "EDGE_TABLE"
model = "EDGE_MODEL"
size = 10
head = f"calc_score_all!(EDGE_TABLE, EDGE_MODEL, {3**size}, {size}, "
tail = ");"

template = "{},{},{},"
text = head
names = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J"
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T"
    "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD",
    "AE", "AF", "AG", "AH", "AI", "AJ", "AK", "AL", "AM", "AN",
    "AO", "AP", "AQ", "AR", "AS", "AT", "AU", "AV", "AW", "AX",
    "AY", "AZ", "BA", "BB", "BC", "BD", "BE", "BF", "BG", "BH",
    "BI", "BJ", "BK", "BL", "BM", "BN", "BO", "BP", "BQ", "BR",
    "BS", "BT", "BU", "BV", "BW", "BX", "BY", "BZ", "CA", "CB",
    "CC", "CD", "CE", "CF", "CG", "CH", "CI", "CJ", "CK", "CL",
    "CM", "CN", "CO", "CP", "CQ", "CR", "CS", "CT", "CU", "CV",
    "CW", "CX", "CY", "CZ", "DA", "DB", "DC", "DD", "DE", "DF",
    "DG", "DH", "DI", "DJ", "DK", "DL", "DM", "DN", "DO", "DP",
]
text = head
n = (3 ** size) // 1000
l = 3 ** size
i = 0
while i < n +1:
    text += template.format(names[i], i * 1000, min(l - i * 1000, 1000))
    i += 1
print(text + tail)

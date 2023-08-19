import re

counts = []
for i in range(256):
    text = "{:08b}".format(i)
    count = 0
    for i in range(8):
        if text[i] == "1":
            count += 1
        else:
            break
    text ="".join(list(reversed(text)))
    for i in range(8):
        if text[i] == "1":
            count += 1
        else:
            break
    if count > 8:
        count = 8

    counts.append(count)

print(counts)
    

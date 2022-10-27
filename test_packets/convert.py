#!/usr/bin/env python

from sys import argv, exit

if len(argv) != 3:
    print(f"Usage: {argv[0]} <infile> <outfile>")
    exit(1)

with open(argv[1], "r") as f:
    lines = [x.strip() for x in f.readlines()]

packets = []
p = []

for l in lines:
    if "peer0" in l:
        p.append(0)
    elif "peer1" in l:
        p.append(1)
    elif "}" in l:
        p += eval("[" + l[:-3] + "]")
        packets.append(p)
        p = []
    else:
        p += eval("[" + l[:-1] + "]")

sep = -1
for x in range(256):
    if not any(list(map(lambda y: x in y, packets))):
        sep = x
        break

if sep >= 0:
    print(f"Found a seperator: {sep}. Writing to disc.")
    with open(argv[2], "wb") as f:
        for p in packets:
            f.write(bytearray(p))
            f.write(bytearray([sep]))
else:
    print("Sadly all bytes are used in the packet, making a seperator ambiguous.")

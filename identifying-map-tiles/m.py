import sys

additions = {
    "0": 0,
    "1": 1,
    "2": 1j,
    "3": 1 + 1j,
}


def solve(s):
    level = 0
    coord = 0 + 0j

    for digit in s:
        level += 1
        coord = coord * 2
        coord += additions[digit]

    return level, coord

level, coord = solve(sys.stdin.read().strip())

print(f"{level} {int(coord.real)} {int(coord.imag)}")


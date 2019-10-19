import sys

def parse_line(l):
    s = l.split()
    return int(s[0]), float(s[1])

inp = list(map(parse_line,  sys.stdin.readlines()[1:]))

def solve(case):
    b, p = case
    bpm = 60 * b / p
    max_bpm = 60 * (b + 1) / p
    min_bpm = 60 * (b - 1) / p

    return f"{min_bpm} {bpm} {max_bpm}"

print("\n".join(map(solve, inp)))

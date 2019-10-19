def max_perfect(x):
    step = 2 if x <= 0 else 1
    x = -x if x <= 0 else x

    perfects = list(
        filter(
            lambda t: int(round(t[1]) ** t[0]) == x,
            map(lambda i: (i, x ** (1/i)),
                range(1, 33, step)
               )
        )
    )

    return str(perfects[-1][0])


if __name__ == "__main__":
    import sys

    vals = map(int, sys.stdin.readlines()[:-1])

    print("\n".join(map(max_perfect, vals)))


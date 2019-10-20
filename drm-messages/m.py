import sys

def value(char):
    return ord(char) - ord("A")

def char(value):
    return chr(value + ord("A"))

def divide(message):
    point = int(len(message) / 2)
    return message[:point], message[point:]

def rotate(value, increment):
    return (value + increment) % (ord("Z") - ord("A") + 1)

def rotate_message(value_message):
    length = sum(value_message)
    return tuple(rotate(value, length) for value in value_message)

if __name__ == "__main__":
    drm_message = tuple(map(value, sys.stdin.read().strip()))

    rotated_pairs = tuple(zip(*map(rotate_message, divide(drm_message))))

    answer = "".join(map(lambda pair: char(rotate(pair[0], pair[1])), rotated_pairs))
    print(answer)


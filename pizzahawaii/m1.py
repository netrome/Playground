import itertools as it
import re
import sys

inp = sys.stdin.readlines()
cases = inp[1:]

def starts_with_int(s):
    return bool(re.match(r"\d+.*", s))

case_idxes = list(map(lambda i: i[0], filter(lambda i: re.match(r"\d+\n", i[1]), enumerate(cases))))

def parse_test_case(case_idx):
    num_cases = int(cases[case_idx])
    idx = case_idx + 1
    test_case = {}

    def process_ingredients(s):
        return set(map(lambda i: i.strip(), s.split(" ")[1:]))

    for _ in range(num_cases):
        test_case[cases[idx].strip()] = tuple(map(process_ingredients, (cases[idx + 2], cases[idx + 1])))
        idx += 3

    return test_case


#print(case_idxes)
test_cases = list(map(parse_test_case, case_idxes))
#print(test_cases)
#print("-----")

# -------- Actually solve this

def get_all_ingredients(test_case):
    all_ingredients = set()
    all_german = set()

    for ing, ger in test_case.values():
        all_ingredients.update(ing)
        all_german.update(ger)

    return all_ingredients, all_german

#all_ing, all_ger = get_all_ingredients(test_cases[0])
#print(all_ing)

def get_vectors(test_case):
    all_ing, all_ger = get_all_ingredients(test_case)
    keys = tuple(test_case.keys())

    ger_vectors = {}
    en_vectors = {}

    for ger in all_ger:
        ger_vectors[ger] = tuple(map(lambda key: ger in test_case[key][1], keys))

    for ing in all_ing:
        en_vectors[ing] = tuple(map(lambda key: ing in test_case[key][0], keys))

    return en_vectors, ger_vectors

def compute_answer(test_case):
    en_vectors, ger_vectors = get_vectors(test_case)

    out = list(filter(lambda t: ger_vectors[t[0]] == en_vectors[t[1]], it.product(ger_vectors, en_vectors)))
    out = sorted(out)
    return "\n".join(map(lambda t: f'({", ".join(t)})', out)) + "\n"

answers = "\n".join(map(compute_answer, test_cases))
sys.stdout.write(answers)

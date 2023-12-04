import sys
import re

LINE_RE = re.compile(r'Card\s+\d+: (?P<left>[\d\s]+)\|(?P<right>[\d\s]+)')

def get_card_matches(lines) -> list[int]:
    matches = []
    for line in lines:
        captures = LINE_RE.match(line.strip())
        left_set = set(s.strip() for s in captures['left'].split(' '))
        right_set = set(s.strip() for s in captures['right'].split(' '))

        matches.append(len(left_set.intersection(right_set)))

    return matches

def problem1(lines):
    print(f"Problem1: {sum(get_card_matches(lines))}")

def problem2(lines):
    matches = dict(enumerate(get_card_matches(lines)))
    counts = {i: 1 for i in matches.keys()}

    for i, match_count in matches.items():
        current_count = counts[i]

        for j in range(i+1, i+match_count):
            counts[j] += current_count

    print(f"Problem2: {sum(counts.values())}")


if __name__ == '__main__':

    with open(sys.argv[1]) as f:
        lines = f.readlines()
        problem1(lines)
        problem2(lines)


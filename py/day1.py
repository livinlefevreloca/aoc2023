import re
import sys

def problem1(path):
    return sum(
            int(
                [
                    c for c in s.strip() if c.isdigit()
                ][0] + [
                    c for c in s.strip() if c.isdigit()
                ][-1]
            )  for s in open(path)
    )

def problem2(path):

    def to_num(s):
        if s.isdigit():
            return s
        return {
                'one': '1',
                'two': '2',
                'three': '3',
                "four": '4',
                "five": '5',
                "six": '6',
                "seven": '7',
                "eight": '8',
                "nine": '9'
        }[s]

    return sum(
        # Use lookahead to find all matches including written numbers that overlap
        int(
            to_num(re.findall(r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))', s)[0])
            + to_num(re.findall(r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))', s)[-1])
        ) for s in open(path)
    )


if __name__ == '__main__':
    print("Problem 1 solution: ", problem1(sys.argv[1]))
    print("Problem 2 solution: ", problem2(sys.argv[1]))

import collections
import sys


def part1(lines):
    counts = collections.defaultdict(int)
    for line in lines:
        for count in set(collections.Counter(line.strip()).values()):
            counts[count] += 1
    return counts[2] * counts[3]


def part2(lines):
    for line1 in lines:
        for line2 in lines:
            diff = False
            for c1, c2 in zip(line1, line2):
                if c1 != c2:
                    if diff:
                        break
                    else:
                        diff = True
            else:
                if diff:
                    return ''.join(c1 for c1, c2 in zip(line1, line2)
                                   if c1 == c2)


def main():
    lines = list(sys.stdin)
    print part1(lines)
    print part2(lines)

if __name__ == '__main__':
    main()

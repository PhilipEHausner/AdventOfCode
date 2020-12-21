import collections

import pyformlang.cfg as pycfg

from get_input import get_input_from_file


if __name__ == '__main__':
    data = collections.deque(get_input_from_file("problem19.txt"))

    rules = {}

    productions = set()

    while line := data.popleft():
        left, right = line.split(":")
        right = right.strip().split("|")

        left = pycfg.Variable(left)

        for subrule in right:
            subrule = subrule.strip().split()
            rules = []
            for el in subrule:
                if el.startswith("\""):
                    el = pycfg.Terminal(el.replace("\"", ""))
                else:
                    el = pycfg.Variable(el)
                rules.append(el)
            productions.add(pycfg.Production(left, rules))

    cfg = pycfg.CFG(productions=productions, start_symbol=pycfg.Variable("0"))

    count = 0
    while len(data) and (line := data.popleft()):
        if cfg.contains(line):
            count += 1

    print(count)


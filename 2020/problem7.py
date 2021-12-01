import re

from get_input import get_input_prob7


def num_of_bags_in_bag(bag_color: str, combs: dict) -> int:
    res = 1
    for holdings in combs[bag_color]:
        res += holdings[0] * num_of_bags_in_bag(holdings[1], combs)
    return res


if __name__ == '__main__':
    possibilities = get_input_prob7()

    possibilities = [poss.strip() for poss in possibilities.split("\n") if poss.strip()]

    combs = {}

    for poss in possibilities:
        holderstr, holdingsstr = poss.split(" contain ")

        holder = re.match(r"(.+)bags?$", holderstr).groups()[0].strip()

        holdings = []
        for res in re.findall(r"(?P<num>\d+)(?P<type>[^,]+) bag", holdingsstr):
            res = (int(res[0]), res[1].strip())
            holdings.append(res)

        combs[holder] = holdings

    valids = {"shiny gold"}
    lastlen = 0

    while True:
        if lastlen == len(valids):
            break
        lastlen = len(valids)

        addings = []
        for holder, holdings in combs.items():
            for valid in valids:
                for holding in holdings:
                    if holding[1] == valid:
                        addings.append(holder)
        for a in addings:
            valids.add(a)

    res = num_of_bags_in_bag("shiny gold", combs) - 1



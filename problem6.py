from get_input import get_input_prob6

if __name__ == '__main__':
    groups = get_input_prob6()

    groups = groups.split("\n\n")
    groups = [" ".join(group.split()) for group in groups if group]

    answers = "abcdefghijklmnopqrstuvwxyz"

    counts = []

    for group in groups:
        c = 0
        for answer in answers:
            if all([answer in g for g in group.split()]):
                c += 1
        counts.append(c)

    print(sum(counts))

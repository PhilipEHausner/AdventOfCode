def parse_argument(arg: str) -> int:
    if arg[0] == "+":
        res = int(arg[1:])
    else:
        res = -1 * int(arg[1:])
    return res


if __name__ == '__main__':
    with open("input/problem8.txt", "r") as f:
        data = f.readlines()
        data = [d.strip() for d in data]

    terminated = False
    accumulator = 0

    for i in range(len(data)):
        curr_data = data.copy()
        accumulator = 0
        curr_line = 0
        executed_lines = set()

        if curr_data[i][:3] == "acc":
            continue

        elif curr_data[i][:3] == "nop":
            curr_data[i] = "jmp" + curr_data[i][3:]

        else:
            curr_data[i] = "nop" + curr_data[i][3:]

        while curr_line not in executed_lines:
            executed_lines.add(curr_line)

            op, arg = curr_data[curr_line].split()

            if op == "nop":
                curr_line += 1

            elif op == "acc":
                curr_line += 1
                accumulator += parse_argument(arg)

            elif op == "jmp":
                curr_line += parse_argument(arg)

            else:
                print(f"Unknown operation {op}.")

            if curr_line == len(curr_data):
                terminated = True
                break

        if terminated:
            break

    print(accumulator)



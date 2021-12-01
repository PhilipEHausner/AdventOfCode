import re

from get_input import get_input_from_file


if __name__ == '__main__':
    data = get_input_from_file("problem14.txt")

    mask = "X" * 36
    memory = {}

    for num, line in enumerate(data):
        print(num)
        if line.startswith("mask"):
            mask = line[7:]
        elif line.startswith("mem"):
            mem_address = int(re.findall(r"\[(\d+)\]", line)[0])
            value = int(re.findall(r"= (\d+)", line)[0])
            binary_val = bin(mem_address)[2:]
            if len(binary_val) > 36:
                print("input too long")
            binary_val = "0" * (36 - len(binary_val)) + binary_val

            new_vals = [""]
            for m, b in zip(mask, binary_val):
                if m == "0":
                    for i, nv in enumerate(new_vals):
                        new_vals[i] += b
                elif m == "1":
                    for i, nv in enumerate(new_vals):
                        new_vals[i] += m
                elif m == "X":
                    newer_vals = [item for subl in [[val + "0", val + "1"] for val in new_vals] for item in subl]
                    new_vals = newer_vals
                else:
                    print("Incorrect mask value.")

            for new_val in new_vals:
                memory[int(new_val, 2)] = value
         #   memory[mem_address] = int(new_val, 2)

        else:
            print("ALARM")

    print(sum(memory.values()))

from get_input import get_input_from_file


if __name__ == '__main__':
    data = get_input_from_file("problem9.txt")
    data = [int(d) for d in data]

    code = 0

    for i in range(25, len(data)):
        preamble = data[i-25:i]

        curr_number = data[i]
        valid = False

        for x in range(len(preamble)):
            for y in range(x+1, len(preamble)):
                if preamble[x] + preamble[y] == curr_number:
                    valid = True
                    break
            if valid:
                break

        if not valid:
            code = curr_number
            break

    found_weakness = False
    elements = []
    for i in range(len(data)):
        curr_index = i
        checksum = 0
        elements = []
        while True:
            elements.append(data[curr_index])
            checksum += data[curr_index]
            curr_index += 1

            if checksum == code:
                found_weakness = True
                break

            if checksum > code:
                break

        if found_weakness:
            break

    print(min(elements) + max(elements))

import re

from get_input import get_input


if __name__ == '__main__':
    passports = get_input()
    passports = [i for i in passports.split("\n\n") if i]
    passports = [re.sub("\n", " ", inp).strip() for inp in passports]

    checks = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]

    valids = 0
    for inp in passports:
        inp = " ".join(inp.split())
        val = True
        for check in checks:
            if check not in inp:
                val = False
                break

        fields = inp.split(" ")

        try:
            for field in fields:
                key, value = field.split(":")
                if key == "byr":
                    if not 1920 <= int(value) <= 2002:
                        print(key, value)
                        val = False
                        break
                if key == "iyr":
                    if not 2010 <= int(value) <= 2020:
                        print(key, value)
                        val = False
                        break
                if key == "eyr":
                    if not 2020 <= int(value) <= 2030:
                        print(key, value)
                        val = False
                        break
                if key == "hgt":
                    if value[-2:] == "in":
                        if not 59 <= int(value[:2]) <= 76:
                            print(key, value)
                            val = False
                            break
                    elif value[-2:] == "cm":
                        if not 150 <= int(value[:3]) <= 193:
                            print(key, value)
                            val = False
                            break
                    else:
                        print(key, value)
                        val = False
                        break
                if key == "hcl":
                    if value[0] != "#":
                        print(key, value)
                        val = False
                        break
                    for i in range(1, len(value)):
                        if value[i] not in "0123456789abcdef":
                            print(key, value)
                            val = False
                            break
                if key == "ecl":
                    if value not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
                        print(key, value)
                        val = False
                        break
                if key == "pid":
                    if len(value) != 9:
                        print(key, value)
                        val = False
                        break
                    for v in value:
                        if v not in "0123456789":
                            print(key, value)
                            val = False
                            break
        except Exception as e:
            val = False

        if val:
            valids += 1


    print(valids)
from typing import List


def get_input_from_file(file: str) -> List[str]:
    with open(file, "r") as f:
        data = f.readlines()
        data = [d.strip() for d in data]
    return data

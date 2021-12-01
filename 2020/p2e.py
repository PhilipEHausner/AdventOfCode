import collections
import math
from typing import Dict


def get_damage_rolls_6d6(half: bool = False) -> Dict[int, float]:
    total = math.pow(6, 6)
    results = [sum([a, b, c, d, e, f]) // 2 if half else sum([a, b, c, d, e, f])
               for a in range(1, 7)
               for b in range(1, 7)
               for c in range(1, 7)
               for d in range(1, 7)
               for e in range(1, 7)
               for f in range(1, 7)]

    counter = collections.Counter(results)
    for key in counter.keys():
        counter[key] /= total

    return counter


def get_cdf_6d6(rolls_probas: Dict) -> Dict:
    lower, upper = min(rolls_probas.keys()), max(rolls_probas.keys())
    result = {}
    curr = 0
    for roll in range(upper, lower - 1, -1):
        curr += rolls_probas[roll]
        result[roll] = curr
    return result


def get_probability_fortitude_save_success(DC: int, save_bonus: int) -> float:
    roll_needed = min(max(DC - save_bonus, 1), 20)

    return 1 - roll_needed / 20 + 0.05


if __name__ == "__main__":
    fort_save = 4
    DC = 20
    # min HP: 6, max HP == 36
    HP = 17

    rolls_probas = get_damage_rolls_6d6()
    lower, upper = min(rolls_probas.keys()), max(rolls_probas.keys())
    rolls_cdf = get_cdf_6d6(rolls_probas)

    rolls_probas_half = get_damage_rolls_6d6(half=True)
    lower_half, upper_half = min(rolls_probas_half.keys()), max(rolls_probas_half.keys())
    rolls_half_cdf = get_cdf_6d6(rolls_probas_half)

    save_success = get_probability_fortitude_save_success(DC, fort_save)

    proba = (1 - save_success) * rolls_cdf[HP]

    if HP <= upper_half:
        proba += save_success * rolls_half_cdf[HP]

    print(proba)


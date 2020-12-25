import collections

from get_input import get_input_from_file


if __name__ == "__main__":
    data = get_input_from_file("problem21.txt")

    allergen_to_food = collections.defaultdict(set)

    all_food_counts = collections.defaultdict(int)

    for line in data:
        ingredients, allergens = line.split(" (contains ")

        ingredients = [i.strip() for i in ingredients.split(" ")]
        for i in ingredients:
            all_food_counts[i] += 1

        allergens = allergens.replace(")", "")
        allergens = [a.strip() for a in allergens.split(",")]

        for allergen in allergens:
            if not allergen_to_food[allergen]:
                allergen_to_food[allergen] = set(ingredients)
            else:
                allergen_to_food[allergen].intersection_update(ingredients)

    safe_foods = {food for food in all_food_counts.keys()}.difference({food for subl in allergen_to_food.values() for food in subl})

    print("Sum of safe foods:", sum([all_food_counts[food] for food in safe_foods]))

    while any([len(subl) > 1 for subl in allergen_to_food.values()]):
        single_foods = {list(el)[0] for el in allergen_to_food.values() if len(el) == 1}
        for allergen in allergen_to_food.keys():
            if len(allergen_to_food[allergen]) != 1:
                allergen_to_food[allergen].difference_update(single_foods)

    bad_ingredients = list(sorted([(a, b.pop()) for a, b in allergen_to_food.items()], key=lambda x: x[0]))
    print(",".join([bad[1] for bad in bad_ingredients]))

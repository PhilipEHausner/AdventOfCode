from typing import List

import numpy as np

from utils.get_input import get_input_from_file


class ImageEnhancer:
    def __init__(self, enhancement_algorithm: List[int]):
        self.enhancement_algorithm = enhancement_algorithm
        self.padding_options = [self.enhancement_algorithm[0], self.enhancement_algorithm[-1]]
        self.padding = 0
        self.swap = True if self.padding != self.padding_options[0] else False

    def _swap_padding(self) -> None:
        if self.swap:
            self.padding = self.padding_options[1] \
                if self.padding == self.padding_options[0] \
                else self.padding_options[0]

    @staticmethod
    def _to_binary(binary_list: List[int]) -> int:
        return int("".join(map(str, binary_list)), 2)

    def _enhancement_step(self, image: np.array) -> np.array:
        image = np.pad(image, 2, mode="constant", constant_values=self.padding)
        new_image = image.copy()

        for i in range(1, len(image) - 1):
            for j in range(1, len(image[i]) - 1):
                sub_image = image[i - 1:i + 2, j - 1:j + 2].flatten()
                algorithm_index = self._to_binary(sub_image)
                new_image[i][j] = self.enhancement_algorithm[algorithm_index]

        new_image = new_image[1:-1, 1:-1]

        self._swap_padding()

        return new_image

    def enhance(self, image: np.array, steps: int) -> np.array:
        for _ in range(steps):
            image = self._enhancement_step(image)
        return image


def part1(image: np.array, enhancement_algorithm: List[int]) -> None:
    image_enhancer = ImageEnhancer(enhancement_algorithm)
    image = image_enhancer.enhance(image, 2)
    print(f"Solution part 1: {np.sum(image)}")


def part2(image: np.array, enhancement_algorithm: List[int]) -> None:
    image_enhancer = ImageEnhancer(enhancement_algorithm)
    image = image_enhancer.enhance(image, 50)
    print(f"Solution part 2: {np.sum(image)}")


def main() -> None:
    data = get_input_from_file("day20.txt")
    enhancement_algorithm = [1 if pixel == "#" else 0 for pixel in data[0]]

    image = []
    for line in data[2:]:
        image.append([1 if pixel == "#" else 0 for pixel in line])
    image = np.vstack(image)

    part1(image.copy(), enhancement_algorithm)
    part2(image.copy(), enhancement_algorithm)


if __name__ == "__main__":
    main()

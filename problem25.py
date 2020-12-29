from get_input import get_input_from_file


def secret_loop(_value: int, _subject_number: int, _max: int) -> int:
    return (_value * _subject_number) % _max


if __name__ == '__main__':
    data = get_input_from_file("problem25.txt")
    card_public_key = int(data[0])
    door_public_key = int(data[1])

    value = 1
    subject_number = 7
    max_remainder = 20201227
    card_loops = 0
    door_loops = 0

    card_secret, door_secret = False, False
    count = 0
    while not card_secret or not door_secret:
        count += 1
        value = secret_loop(value, subject_number, max_remainder)
        if value == card_public_key:
            card_loops = count
            card_secret = True

        if value == door_public_key:
            door_loops = count
            door_secret = True

    encryption_key = 1

    for _ in range(card_loops):
        encryption_key = secret_loop(encryption_key, door_public_key, max_remainder)

    print(encryption_key)

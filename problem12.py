from get_input import get_input_from_file


if __name__ == '__main__':
    data = get_input_from_file("problem12.txt")

    data = [(line[0], int(line[1:])) for line in data]

    position = [0, 0]  # N/S and E/W
    directions = ["E", "N", "W", "S"]
    direction = 0

    waypoint = [1, 10]

    for command, quantity in data:
        if command == "F":
            position[0] += quantity * waypoint[0]
            position[1] += quantity * waypoint[1]

        if command == "N":
            waypoint[0] += quantity
        elif command == "S":
            waypoint[0] -= quantity
        elif command == "W":
            waypoint[1] -= quantity
        elif command == "E":
            waypoint[1] += quantity
        elif command == "L":
            turns = (direction + quantity // 90) % 4
            while turns > 0:
                waypoint = [waypoint[1], -waypoint[0]]
                turns -= 1
        elif command == "R":
            turns = (direction + quantity // 90) % 4
            while turns > 0:
                waypoint = [-waypoint[1], waypoint[0]]
                turns -= 1

    print(f"Manhattan Distance: {sum(map(abs, position))}")

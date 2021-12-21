import functools
from typing import List, Tuple

from utils.get_input import get_input_from_file


def add_to_player_pos(player_pos: int, die_roll: List[int]) -> int:
    player_pos = (player_pos + sum(die_roll)) % 10
    return 10 if player_pos == 0 else player_pos


def three_d_100_roll(base_roll: int) -> List[int]:
    roll = [base_roll % 100, (base_roll + 1) % 100, (base_roll + 2) % 100]
    roll = [100 if die == 0 else die for die in roll]
    return roll


def part1_turn(player_pos: int, player_score: int, base_roll: int) -> Tuple[bool, int, int]:
    player_pos = add_to_player_pos(player_pos, three_d_100_roll(base_roll))
    player_score += player_pos
    winner = False
    if player_score >= 1000:
        winner = True
    return winner, player_pos, player_score


def part1(player1_pos: int, player2_pos: int) -> None:
    player1_score, player2_score = 0, 0
    turn = "player1"
    base_roll = 1

    while True:

        if turn == "player1":
            won, player1_pos, player1_score = part1_turn(player1_pos, player1_score, base_roll)
            if won:
                winner = "player1"
                break
        else:
            won, player2_pos, player2_score = part1_turn(player2_pos, player2_score, base_roll)
            if won:
                winner = "player2"
                break

        turn = "player1" if turn == "player2" else "player2"
        base_roll += 3

    if winner == "player1":
        loser_score = player2_score
    else:
        loser_score = player1_score

    print(f"Solution part 1: {loser_score * (base_roll + 2)}")


@functools.lru_cache(maxsize=20000000)
def split_the_universe(
        player1_pos: int, player2_pos: int, player1_score: int, player2_score: int, r1: int, r2: int, r3: int, turn: str
) -> Tuple[int, int]:
    if turn == "player1":
        player1_pos = add_to_player_pos(player1_pos, [r1, r2, r3])
        player1_score += player1_pos
        turn = "player2"
    elif turn == "player2":
        player2_pos = add_to_player_pos(player2_pos, [r1, r2, r3])
        player2_score += player2_pos
        turn = "player1"

    if player1_score >= 21:
        return 1, 0
    if player2_score >= 21:
        return 0, 1

    player1_wins, player2_wins = 0, 0
    for i in range(3):
        for j in range(3):
            for k in range(3):
                additional_wins = split_the_universe(player1_pos, player2_pos, player1_score, player2_score,
                                                     i + 1, j + 1, k + 1, turn)
                player1_wins, player2_wins = player1_wins + additional_wins[0], player2_wins + additional_wins[1]

    return player1_wins, player2_wins


def part2(player1_pos: int, player2_pos: int) -> None:
    player1_wins, player2_wins = 0, 0
    turn = "player1"
    for i in range(3):
        for j in range(3):
            for k in range(3):

                additional_wins = split_the_universe(player1_pos, player2_pos, 0, 0,
                                                     i + 1, j + 1, k + 1, turn)
                player1_wins, player2_wins = player1_wins + additional_wins[0], player2_wins + additional_wins[1]

    print(f"Solution part2: {max(player1_wins, player2_wins)}")


def main() -> None:
    data = get_input_from_file("day21.txt")

    player1_pos = int(data[0][-1])
    player2_pos = int(data[1][-1])

    part1(player1_pos, player2_pos)
    part2(player1_pos, player2_pos)


if __name__ == "__main__":
    main()

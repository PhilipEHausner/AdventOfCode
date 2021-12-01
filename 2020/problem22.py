import collections

from get_input import get_input_from_file


def check_and_update_game_state(curr_game_state: set, deck1: collections.deque, deck2: collections.deque) -> bool:
    len_d1, len_d2 = len(deck1), len(deck2)
    temp_deck = deck1.copy()
    deck1, deck2 = deck1.copy(), deck2.copy()
    deck1.extend(deck2)
    deck2.extend(temp_deck)
    state1 = tuple(list(deck1) + [0] + list(deck2))
    state2 = tuple(list(deck2) + [0] + list(deck1))
    if state1 in curr_game_state or state2 in curr_game_state:
        return False
    else:
        curr_game_state.update({state1, state2})
    return True


def combat(deck1, deck2) -> str:
    deck1, deck2 = deck1.copy(), deck2.copy()
    while len(deck1) and len(deck2):
        card1, card2 = deck1.popleft(), deck2.popleft()
        if card1 > card2:
            deck1.extend([card1, card2])
        else:
            deck2.extend([card2, card1])

    if len(deck1):
        res = "Player 1"
    else:
        res = "Player 2"
    return res


def recursive_round(deck1, deck2) -> str:
    game_state = set()
    winner = None
    num = 1

    while len(deck1) and len(deck2):

        num += 1
        card1, card2 = deck1.popleft(), deck2.popleft()

        if card1 <= len(deck1) and card2 <= len(deck2):
            subwinner = recursive_round(collections.deque([deck1[i] for i in range(card1)]),
                                        collections.deque([deck2[i] for i in range(card2)]))

            if subwinner == "Player 1":
                deck1.extend([card1, card2])
            else:
                deck2.extend([card2, card1])
        else:
            if card1 > card2:
                deck1.extend([card1, card2])
            else:
                deck2.extend([card2, card1])

        if not check_and_update_game_state(game_state, deck1, deck2):
            winner = "Player 1"
            break

    if not winner and len(deck2):
        winner = "Player 2"
    else:
        winner = "Player 1"

    return winner

if __name__ == "__main__":
    data = get_input_from_file("problem22.txt")

    player1_deck = collections.deque()
    player2_deck = collections.deque()

    curr_deck = player1_deck

    for line in data:
        if not line or line == "Player 1:":
            continue
        elif line == "Player 2:":
            curr_deck = player2_deck
        else:
            curr_deck.append(int(line))

    # while len(player1_deck) and  len(player2_deck):
    #     card1, card2 = player1_deck.popleft(), player2_deck.popleft()
    #     if card1 > card2:
    #         player1_deck.extend([card1, card2])
    #     else:
    #         player2_deck.extend([card2, card1])
    #
    # if len(player1_deck):
    #     winner_deck = player1_deck
    # else:
    #     winner_deck = player2_deck
    #
    # print(sum([card * (len(winner_deck) - i) for i, card in enumerate(winner_deck)]))

    winner = recursive_round(player1_deck, player2_deck)

    if winner == "Player 1":
        winner_deck = player1_deck
    else:
        winner_deck = player2_deck

    print(sum([card * (len(winner_deck) - i) for i, card in enumerate(winner_deck)]))

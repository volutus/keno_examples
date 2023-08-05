import random

payouts = {
    1: [0, 2],
    2: [0, 0, 10],
    3: [0, 0, 2, 23],
    4: [0, 0, 1, 5, 55],
    5: [0, 0, 0, 2, 20, 300],
    6: [0, 0, 0, 1, 6, 55, 1000],
    7: [1, 0, 0, 0, 2, 20, 100, 5000],
    8: [2, 0, 0, 0, 0, 6, 75, 550, 10000],
    9: [2, 0, 0, 0, 0, 5, 20, 125, 3000, 30000],
    10: [5, 0, 0, 0, 0, 2, 10, 45, 300, 5000, 100000]
}

ALL_NUMBERS = range(1, 81)


def play(spots):
    player_numbers = set()
    while len(player_numbers) < spots:
        random_spot = random.randint(1, 80)
        player_numbers.add(random_spot)

    winning_numbers = set()
    while len(winning_numbers) < 20:
        random_spot = random.randint(1, 80)
        winning_numbers.add(random_spot)

    matches = 0
    for spot in player_numbers:
        if spot in winning_numbers:
            matches += 1

    winnings = payouts[spots][matches]
    return winnings

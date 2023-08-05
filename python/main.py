import payouts
import time

ITERATIONS = 1000000
spots = range(1, 11)


def run():
    start = time.time()
    for spot in spots:
        starting_balance = ITERATIONS
        current_balance = starting_balance
        for i in range(0, ITERATIONS):
            current_balance -= 1
            winnings = payouts.play(spot)
            current_balance += winnings

        output = f"""
Results for {spot}-spot:
Starting Balance: {starting_balance}
Current Balance: {current_balance}
GAIN/LOSS: {current_balance - starting_balance}
"""
        print(output)

    end = time.time()
    print("Execution time in seconds: %s ", end - start)


if __name__ == '__main__':
    run()

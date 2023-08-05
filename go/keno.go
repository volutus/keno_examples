package main

import (
	"fmt"
	"math/rand"
	"time"
)

const ITERATIONS = 10000000

var payouts = make(map[int][]int)

func main() {

	start := time.Now()
	var allNumbers [80]int
	for i := 1; i <= 80; i++ {
		allNumbers[i-1] = i
	}

	payouts[1] = []int{0, 2}
	payouts[2] = []int{0, 0, 10}
	payouts[3] = []int{0, 0, 2, 23}
	payouts[4] = []int{0, 0, 1, 5, 55}
	payouts[5] = []int{0, 0, 0, 2, 20, 300}
	payouts[6] = []int{0, 0, 0, 1, 6, 55, 1000}
	payouts[7] = []int{1, 0, 0, 0, 2, 20, 100, 5000}
	payouts[8] = []int{2, 0, 0, 0, 0, 6, 75, 550, 10000}
	payouts[9] = []int{2, 0, 0, 0, 0, 5, 20, 125, 3000, 30000}
	payouts[10] = []int{5, 0, 0, 0, 0, 2, 10, 45, 300, 5000, 100000}

	for spot := 1; spot <= 10; spot++ {
		startingBalance := ITERATIONS
		currentBalance := startingBalance
		for j := 0; j < ITERATIONS; j++ {
			currentBalance--
			winnings := Play(spot)
			currentBalance += winnings
		}
		gainLoss := currentBalance - startingBalance
		output := `
        Results for %d-spot: 
        Starting Balance: %d
        Current Balance: %d
        GAIN/LOSS: %d
        `
		fmt.Printf(output, spot, startingBalance, currentBalance, gainLoss)
	}

	duration := time.Since(start)

	// Formatted string, such as "2h3m0.5s" or "4.503μs"
	fmt.Println(duration)
}

func Play(spot int) int {

	playerNumbers := map[int]bool{}
	for len(playerNumbers) < spot {
		randomSpot := rand.Intn(81)
		playerNumbers[randomSpot] = true
	}

	winningNumbers := map[int]bool{}
	for len(winningNumbers) < 20 {
		randomSpot := rand.Intn(81)
		winningNumbers[randomSpot] = true
	}

	matches := 0
	for key := range playerNumbers {
		_, ok := winningNumbers[key]
		if ok {
			matches += 1
		}
	}

	return payouts[spot][matches]
}

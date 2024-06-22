use std::collections::HashMap;
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;
use std::time::Instant;


const ITERATIONS: i32 = 1000000;
const PAYOUTS: Lazy<HashMap<i32, [i32; 11]>> = Lazy::new(|| 
{     
    let m = HashMap::from([
        (1, [0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 
        (2, [0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0]),
        (3, [0, 0, 2, 23, 0, 0, 0, 0, 0, 0, 0]),
        (4, [0, 0, 1, 5, 55, 0, 0, 0, 0, 0, 0]),
        (5, [0, 0, 0, 2, 20, 300, 0, 0, 0, 0, 0]),
        (6, [0, 0, 0, 1, 6, 55, 1000, 0, 0, 0, 0]),
        (7, [1, 0, 0, 0, 2, 20, 100, 5000, 0, 0, 0]),
        (8, [2, 0, 0, 0, 0, 6, 75, 550, 10000, 0, 0]),
        (9, [2, 0, 0, 0, 0, 5, 20, 125, 3000, 30000, 0]),
        (10, [5, 0, 0, 0, 0, 2, 10, 45, 300, 5000, 100000])
    ]);
    m  
});

fn main() 
{
    let start = Instant::now();
    for spot in 1..11 
    {
        let starting_balance = ITERATIONS;
        let mut current_balance = starting_balance;
        for _ in 0..ITERATIONS
        {
            let winnings = play(spot);
            current_balance = current_balance - 1 + winnings;
        }
        let gain_loss = current_balance - starting_balance;


        println!("Results for {}-spot: 
        Starting Balance: {}
        Current Balance: {}
        GAIN/LOSS: {}", 
        spot, starting_balance, current_balance, gain_loss);
    }
    let end = Instant::now();
    println!("Finished in {} milliseconds", end.duration_since(start).as_millis());
}

fn play(spot: i32) -> i32
{   
    let player_numbers: Vec<usize> = fetch_random_numbers(spot);
    let winning_numbers = fetch_random_numbers(20);
    let matches = winning_numbers.iter().filter(|&&x| player_numbers.contains(&x)).count();
    return PAYOUTS[&spot][matches];
}

fn fetch_random_numbers(count: i32) -> Vec<usize>
{
    let index = count as usize;
    let mut numbers: Vec<usize> = (0..80).collect();
    numbers.shuffle(&mut rand::thread_rng());
    let usable_numbers = numbers[0..index].to_vec();
    return usable_numbers;
}

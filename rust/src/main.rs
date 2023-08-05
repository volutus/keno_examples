use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use once_cell::sync::Lazy;


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
    for spot in 1..11 
    {
        let starting_balance = ITERATIONS;
        let mut current_balance = starting_balance;
        for _ in 0..ITERATIONS
        {
            current_balance -= 1;
            let winnings = play(spot);
            current_balance += winnings
        }
        let gain_loss = current_balance - starting_balance;


        println!("Results for {}-spot: 
        Starting Balance: {}
        Current Balance: {}
        GAIN/LOSS: {}", 
        spot, starting_balance, current_balance, gain_loss);
    }
}

fn play(spot: i32) -> i32
{   
    let mut player_numbers: HashSet<i32> = HashSet::new();
    while player_numbers.len() < usize::try_from(spot).unwrap() {
        let random_spot = rand::thread_rng().gen_range(1..81);
        player_numbers.insert(random_spot);
    }

    let mut winning_numbers: HashSet<i32> = HashSet::new();
    while winning_numbers.len() < 20 {
        let random_spot = rand::thread_rng().gen_range(1..81);
        winning_numbers.insert(random_spot);
    }

    let matches = player_numbers.intersection(&winning_numbers).count();
    return PAYOUTS[&spot][matches];
}

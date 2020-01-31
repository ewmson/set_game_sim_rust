use strum_macros::EnumIter; // etc.
use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::Itertools;
use rayon::prelude::*;
use histogram::Histogram;
use std::convert::TryInto;
use std::collections::HashSet;
use std::iter::FromIterator;



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SetCard {
    color: SetConstraintOption,
    shading: SetConstraintOption,
    shape: SetConstraintOption,
    number: SetConstraintOption,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
enum SetConstraintOption {
    ONE = 1,
    TWO= 2,
    THREE= 3
}

fn play_game(mut deck: Vec<SetCard>) -> usize{
    let mut board = HashSet::from_iter(deck.split_off(deck.len()-12));
    loop{
        let sets = find_sets(&board);
        if sets.is_empty() && deck.is_empty(){
            return board.len();
        }
        if !sets.is_empty(){
            let chosen_set = choose_set(&sets, &board);
            assert!(sets.contains(&chosen_set));
            board.remove(&chosen_set.0);
            board.remove(&chosen_set.1);
            board.remove(&chosen_set.2);
        }
        if !deck.is_empty() {
            board.extend(deck.split_off(deck.len()-3));
        }
    }
}
fn choose_set(sets: &HashSet<(SetCard, SetCard, SetCard)>,_board: &HashSet<SetCard>) -> (SetCard, SetCard, SetCard){
     *sets.iter().nth(0).unwrap()
}
fn find_sets(board: &HashSet<SetCard>) -> HashSet<(SetCard, SetCard, SetCard)>{
    board.iter().tuple_combinations().filter(|(one, two, three)| is_set(&one, &two, &three)).map(|(one,two,three)| (*one,*two,*three)).collect()
}
fn is_set(one: &SetCard, two: &SetCard, three: &SetCard) -> bool {
    // yeah I know, we are not doing bitwise ops yet.
    satisfy_set(&one.color, &two.color, &three.color) && satisfy_set(&one.shading, &two.shading, &three.shading) &&
    satisfy_set(&one.shape, &two.shape, &three.shape) && satisfy_set(&one.number, &two.number, &three.number)
}
fn satisfy_set(one: &SetConstraintOption, two: &SetConstraintOption, three: &SetConstraintOption) -> bool{
    (*one == *two && *one == *three) || (*one != *two && *one != *three && *two != *three)
}
fn generate_deck() -> Vec<SetCard>{
    let mut deck = vec![];
    for color in SetConstraintOption::iter() {
        for shading in SetConstraintOption::iter() {
            for shape in SetConstraintOption::iter() {
                for number in SetConstraintOption::iter() {
                    deck.push(SetCard{color, shading, shape, number});
                }
            }
        }
    }
    deck
}

fn main() {
    let max_games = 100000;
    let leftover_cards = (1..max_games).into_par_iter().map(|_|{
        let mut deck = generate_deck();
        deck.shuffle(&mut thread_rng());
        return play_game(deck);
    });
    let mut histogram = Histogram::new();
    for value in leftover_cards.collect::<Vec<_>>(){
        histogram.increment(value.try_into().unwrap()).unwrap();
    }
    println!("games won is {} out of {} avg={} cards wins={:.2}% 6={:.2}% 9={:.2}% 12={:.2}% 15={:.2}% 18={:.2}%",
        histogram.get(0).unwrap(),
        max_games,
        histogram.mean().unwrap(),
        (histogram.get(0).unwrap() as f64 / max_games as f64)*100.0,
        (histogram.get(6).unwrap() as f64 / max_games as f64)*100.0,
        (histogram.get(9).unwrap() as f64 / max_games as f64)*100.0,
        (histogram.get(12).unwrap() as f64 / max_games as f64)*100.0,
        (histogram.get(15).unwrap() as f64 / max_games as f64)*100.0,
        (histogram.get(18).unwrap() as f64 / max_games as f64)*100.0,
    );
}

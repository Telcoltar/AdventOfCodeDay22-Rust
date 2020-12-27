mod test_main;

use std::collections::{VecDeque, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn get_input_data(filename: &str) -> (VecDeque<i32>, VecDeque<i32>) {
    let f = File::open(filename).unwrap();
    let mut f = BufReader::new(f);

    let player_1 = parse_player(&mut f);

    let player_2 = parse_player(&mut f);

    return (player_1, player_2);
}

fn parse_player(file: &mut BufReader<File>) -> VecDeque<i32> {
    let current_line = &mut "".to_string();

    let mut _bytes_read = file.read_line(current_line);
    current_line.clear();
    _bytes_read = file.read_line(current_line);

    let mut player = VecDeque::new();
    while current_line.trim() != "" {
        player.push_back(current_line.trim().parse().unwrap());
        current_line.clear();
        _bytes_read = file.read_line(current_line);
    }
    return player;
}

fn play_game(player_1: &mut VecDeque<i32>,player_2: &mut VecDeque<i32>) -> i32 {
    while player_1.len() > 0 && player_2.len() > 0 {
        debug!("{:?}, {:?}", player_1, player_2);
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();
        if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    return if player_1.len() > 0 {
        0
    } else {
        1
    }
}

fn slice_deque(player: VecDeque<i32>, index: i32) -> VecDeque<i32> {
    let mut new_player = VecDeque::new();
    for i in 0..index {
        new_player.push_back(player[i as usize])
    }
    return new_player;
}

fn play_recursive_game(player_1: &mut VecDeque<i32>,player_2: &mut VecDeque<i32>) -> i32 {
    let mut mem: HashSet<VecDeque<i32>> = HashSet::new();
    while player_1.len() > 0 && player_2.len() > 0 {
        if mem.contains(player_1) {
            return 0;
        }
        mem.insert(player_1.clone());
        debug!("{:?}, {:?}", player_1, player_2);
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();
        let winner;
        if player_1.len() >= card_1 as usize && player_2.len() >= card_2 as usize {
            let mut new_player_1 = slice_deque(player_1.clone(), card_1);
            let mut new_player_2 = slice_deque(player_2.clone(), card_2);
            winner = play_recursive_game(&mut new_player_1, &mut new_player_2)
        } else {
            if card_1 > card_2 {
                winner = 0;
            } else {
                winner = 1;
            }
        }
        if winner == 0 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    return if player_1.len() > 0 {
        0
    } else {
        1
    }
}

fn compute_score(mut player: VecDeque<i32>) -> i32 {
    let mut score = 0;
    let mut current_weight = 1;
    while player.len() > 0 {
        score += player.pop_back().unwrap() * current_weight;
        current_weight += 1;
    }
    return score;
}

fn get_score_from_winner(winner: i32, player_1: VecDeque<i32>, player_2: VecDeque<i32>) -> i32 {
    let winning_player;
    if winner == 0 {
        winning_player = player_1;
    } else {
        winning_player = player_2;
    }
    return compute_score(winning_player);
}

fn solution_part_1(filename: &str) -> i32 {
    let (mut player_1, mut player_2) = get_input_data(filename);

    let winner = play_game(&mut player_1, &mut player_2);
    debug!("{:?}, {:?}", player_1, player_2);
    return get_score_from_winner(winner, player_1, player_2);
}

fn solution_part_2(filename: &str) -> i32 {
    let (mut player_1, mut player_2) = get_input_data(filename);
    let winner = play_recursive_game(&mut player_1,&mut player_2);
    debug!("{:?}, {:?}", player_1, player_2);
    debug!("{:?}", winner);
    return get_score_from_winner(winner, player_1, player_2);
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}

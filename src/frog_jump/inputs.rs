use super::places::Place;
use rand::prelude::*;
use std::collections::HashSet;

// starting from 0
// create equally spaced chains all advancing at speed
pub fn chains(speed: i32, chains_number: usize) -> Vec<i32> {
    // start by accelerating at wanted speed
    let mut stones = (0..=speed).map(|s| s * (s + 1) / 2).collect::<Vec<_>>();

    // now, we need one stone path which leaves the last chain towards the next one
    // all while keeping expanding the chains
    let end = stones.last().copied().unwrap();

    let distance = speed as usize / chains_number;
    stones.extend(
        std::iter::successors(Some(vec![end + speed, end + speed + 1]), |alive_stones| {
            let n = alive_stones.len();
            let (chains, mover) = alive_stones.split_at(n - 1);
            if chains.len() == chains_number {
                None // we have enough
            } else {
                let next_mover = mover[0] + speed + 1;
                let mut next_chains = chains.iter().map(|s| s + speed).collect::<Vec<_>>();
                let new_chain_start = next_chains.last().cloned().unwrap() + distance as i32;
                if next_mover - 1 == new_chain_start {
                    next_chains.push(new_chain_start)
                }
                next_chains.push(next_mover);
                Some(next_chains)
            }
        })
        .flatten(),
    );
    let n = stones.len();
    let chains = &stones[n - chains_number - 1..(n - 1)];
    let mut last_chains = chains.iter().map(|s| s + speed).collect::<Vec<_>>();
    stones.append(&mut last_chains);
    stones
}

pub fn trap(speed: i32, chains_number: usize) -> Vec<i32> {
    let mut stones = chains(speed, chains_number);
    let n = stones.len();
    let distance = speed as usize / chains_number;
    let mut chains: Vec<i32> = stones[n - chains_number..].to_owned();
    let middle_chain = chains_number / 2;
    let mut diamonds_order = (0..chains_number)
        .filter(|&c| c != middle_chain)
        .collect::<Vec<_>>();
    diamonds_order.shuffle(&mut rand::thread_rng());
    diamonds_order.insert(0, middle_chain);
    let width = distance - 3;
    let mut completed_chains = std::collections::HashSet::new();
    while let Some(chain_to_diamond) = diamonds_order.pop() {
        let advance = distance + 1; // how many times should we advance while another chain diamonds
        let new_chains = chains
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if i == chain_to_diamond {
                    completed_chains.insert(i);
                    stones.append(&mut diamond(c, speed, width))
                } else {
                    if !completed_chains.contains(&i) {
                        stones.extend(
                            std::iter::successors(Some(c), |c| Some(c + speed)).take(advance),
                        );
                    }
                }
                stones.last().copied().unwrap()
            })
            .collect();
        chains = new_chains;
    }
    stones.sort_unstable();
    stones
}

fn diamond(starting_stone: i32, speed: i32, width: usize) -> Vec<i32> {
    let mut stones = Vec::new();
    let mut range = (starting_stone + speed - 1)..(starting_stone + speed + 2);
    for _ in 0..width / 2 {
        stones.extend(range.clone());
        range.start += speed - 1;
        range.end += speed + 1;
    }
    while !range.is_empty() {
        stones.extend(range.clone());
        range.start += speed + 1;
        range.end += speed - 1;
    }
    stones
}

pub fn random_input(n: usize) -> Vec<i32> {
    std::iter::once(0)
        .chain(std::iter::repeat(1).take(4))
        .chain(std::iter::repeat_with(|| {
            (rand::random::<u8>() % 2 + 1) as i32
            //rand::random::<u8>().leading_ones() as i32 + 1
        }))
        .take(n - 1)
        .scan(0, |pos, diff| {
            *pos += diff;
            Some(*pos)
        })
        .chain(std::iter::once(std::i32::MAX))
        .collect()
}

pub fn smart_random_input() -> Vec<i32> {
    // we start by accelerating at max speed
    let places = std::iter::successors(
        Some(Place {
            position: 0,
            speed: 0,
        }),
        |p| p.next_places().last(),
    )
    .take(800)
    .collect::<Vec<_>>();
    // now we compute our target position
    let start = places.last().copied().unwrap();
    let k = 160;
    let d = 30;
    // we want to keep speed k times
    // and decelerate d times
    // let v be the starting speed
    // we end up moving by k*v + v-1 + v-2 + v-3...
    // which equals v(k+d) - d(d+1) / 2
    let before_target = Place {
        position: start.position + start.speed * (k + d) - d * (d + 1) / 2,
        speed: start.speed - d,
    };
    let target_position = before_target.position + before_target.speed;
    let mut previous_places = vec![start];
    let mut seen_places = HashSet::new();
    let mut stones = places
        .into_iter()
        .map(|p| p.position)
        .collect::<HashSet<_>>();
    let mut forbidden_positions = HashSet::new();
    while !stones.contains(&target_position) {
        let next_places = previous_places
            .into_iter()
            // by taking the two first places we ensure no acceleration
            .flat_map(move |p| p.next_places().take(2))
            .filter(|&p| {
                p.position <= target_position
                    && seen_places.insert(p)
                    && !forbidden_positions.contains(&p.position)
                    && (p == before_target || {
                        let ok = p.position < before_target.position
                            || p.next_places()
                                .find(|&p| p.position == target_position)
                                .is_none();
                        if !ok {
                            forbidden_positions.insert(p.position);
                        }
                        ok
                    })
            })
            .inspect(|p| {
                stones.insert(p.position);
            })
            .collect::<Vec<_>>();
        assert!(!next_places.is_empty());
        previous_places = next_places
    }
    let mut stones = stones.into_iter().collect::<Vec<_>>();
    stones.sort();
    eprintln!("we got {} stones", stones.len());
    stones
}

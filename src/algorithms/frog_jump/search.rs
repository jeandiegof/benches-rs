use diam::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, LinkedList};

use super::{marks::ParHashTable, places::Place};

pub fn can_cross_par<H: ParHashTable>(stones: &[i32]) -> bool {
    let starting_place = Place {
        position: 0,
        speed: 0,
    };

    let mut remaining_places = vec![starting_place];

    let hashed_stones = stones
        .iter()
        .enumerate()
        .map(|(index, position)| (*position, index))
        .collect::<HashMap<_, _>>();

    let target_position = stones.last().copied().unwrap_or_default();
    let seen_places = H::new(stones);

    while !remaining_places.is_empty() {
        let end_size = remaining_places.len().min(30);
        let end_start = remaining_places.len() - end_size;
        let new_places: Result<Vec<Place>, ()> = remaining_places
            .par_drain(end_start..)
            .rev() // someone starts by the end like the sequential algorithm
            .map(|e| {
                let mut stack = vec![e];
                let found_path = sequential_exploration(
                    3000,
                    &mut stack,
                    target_position,
                    &hashed_stones,
                    &seen_places,
                );
                if found_path {
                    Err(())
                } else {
                    Ok(stack)
                }
            })
            .map(|rv| rv.map(|v| std::iter::once(v).collect::<LinkedList<_>>()))
            .log("foo")
            .try_reduce(LinkedList::new, |mut l1, mut l2| {
                l2.append(&mut l1); // reverse to get results in correct order
                Ok(l2)
            })
            .map(|l| {
                l.into_iter()
                    .reduce(|mut v1, mut v2| {
                        v1.append(&mut v2);
                        v1
                    })
                    .unwrap_or_default()
            });
        match new_places {
            Err(()) => return true,
            Ok(mut v) => remaining_places.append(&mut v),
        }
    }
    false
}

fn sequential_exploration<H: ParHashTable>(
    number_of_places_to_explore: usize,
    remaining_places: &mut Vec<Place>,
    target_position: i32,
    hashed_stones: &HashMap<i32, usize>,
    seen_places: &H,
) -> bool {
    for _ in 0..number_of_places_to_explore {
        if let Some(current_place) = remaining_places.pop() {
            if current_place.position == target_position {
                return true;
            }
            remaining_places.extend(
                current_place
                    .next_places_hashmap(&hashed_stones)
                    .filter(|&place| {
                        !seen_places.insert(hashed_stones[&place.position], place.speed)
                    }),
            )
        } else {
            return false;
        }
    }
    false
}

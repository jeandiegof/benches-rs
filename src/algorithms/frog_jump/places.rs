use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Place {
    pub position: i32,
    pub speed: i32,
}

impl Place {
    pub fn next_places_hashmap<'s>(
        &'s self,
        hashed_stones: &'s HashMap<i32, usize>,
    ) -> impl Iterator<Item = Place> + 's {
        ((self.speed - 1).max(0)..(self.speed + 2)).filter_map(move |speed| {
            let next_position = self.position + speed;
            if hashed_stones.contains_key(&next_position) {
                Some(Place {
                    position: next_position,
                    speed,
                })
            } else {
                None
            }
        })
    }
}

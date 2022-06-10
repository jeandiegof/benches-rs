use std::sync::atomic::{AtomicBool, Ordering};

pub trait SeqHashTable: Sized {
    // return a new table with given capacity
    fn new(stones: &[i32]) -> Self;
    // insert a new key, returning if it was already there
    fn insert_mut(&mut self, stone_index: usize, speed: i32) -> bool;
}

pub trait ParHashTable: SeqHashTable + Sync + Send {
    fn insert(&self, stone_index: usize, speed: i32) -> bool;
}

impl SeqHashTable for AtomicMarks {
    fn new(stones: &[i32]) -> Self {
        AtomicMarks::for_stones(stones)
    }
    fn insert_mut(&mut self, stone_index: usize, speed: i32) -> bool {
        self.mark(stone_index, speed)
    }
}

impl ParHashTable for AtomicMarks {
    fn insert(&self, stone_index: usize, speed: i32) -> bool {
        self.mark(stone_index, speed)
    }
}

impl SeqHashTable for scc::HashSet<(usize, i32)> {
    fn new(stones: &[i32]) -> Self {
        use std::collections::hash_map::RandomState;
        scc::HashSet::new(stones.len() * 10, RandomState::new())
    }
    fn insert_mut(&mut self, stone_index: usize, speed: i32) -> bool {
        self.insert((stone_index, speed)).is_err()
    }
}

impl ParHashTable for scc::HashSet<(usize, i32)> {
    fn insert(&self, stone_index: usize, speed: i32) -> bool {
        self.insert((stone_index, speed)).is_err()
    }
}

impl SeqHashTable for Marks {
    fn new(stones: &[i32]) -> Self {
        Marks::for_stones(stones)
    }
    fn insert_mut(&mut self, stone_index: usize, speed: i32) -> bool {
        self.mark(stone_index, speed)
    }
}

impl SeqHashTable for std::collections::HashSet<(usize, i32)> {
    fn new(stones: &[i32]) -> Self {
        std::collections::HashSet::with_capacity(stones.len() * 10)
    }
    fn insert_mut(&mut self, stone_index: usize, speed: i32) -> bool {
        !self.insert((stone_index, speed))
    }
}

pub struct AtomicMarks {
    marks: Vec<AtomicBool>,
    max_speed: usize,
}

impl AtomicMarks {
    pub fn for_stones(stones: &[i32]) -> Self {
        let max_speed = stones.len();
        let size = std::mem::size_of::<AtomicBool>() * max_speed * stones.len();
        let marks = vec![0u8; size];
        AtomicMarks {
            marks: unsafe { std::mem::transmute(marks) },
            max_speed,
        }
    }
    pub fn mark(&self, stone_index: usize, speed: i32) -> bool {
        self.marks[stone_index * self.max_speed + speed as usize].swap(true, Ordering::SeqCst)
    }
}

pub struct Marks {
    marks: Vec<bool>,
    max_speed: usize,
}

impl Marks {
    pub fn for_stones(stones: &[i32]) -> Self {
        let max_speed = stones.len();
        let size = max_speed * stones.len();
        let marks = vec![false; size];
        Marks { marks, max_speed }
    }
    pub fn mark(&mut self, stone_index: usize, speed: i32) -> bool {
        let i = stone_index * self.max_speed + speed as usize;
        let old = self.marks[i];
        self.marks[i] = true;
        old
    }
}

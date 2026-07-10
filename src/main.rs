use std::{
    array, cmp::Reverse, collections::{BinaryHeap, HashMap},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let ans = a_star(State::start(), State::target()).context("no path found")?;
    println!("{}", ans);

    let ans = a_star(State::start_part2(), State::target_part2()).context("no path found")?;
    println!("{}", ans);

    Ok(())
}

const NUM_FLOORS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    elevator: u8,
    microchips: [u8; NUM_FLOORS],
    generators: [u8; NUM_FLOORS],
}

/// Find the shortest path from source to target.
fn a_star(source: State, target: State) -> Option<u8> {
    let mut seen = HashMap::new();
    let mut to_visit = BinaryHeap::new(); // min-heap

    let t = source.target_estimate();
    seen.insert(source.clone(), t);
    to_visit.push((Reverse(0 + t), 0, source));

    while let Some((Reverse(cost), source_est, curr)) = to_visit.pop() {
        if seen[&curr] < cost {
            continue;
        }

        if curr == target {
            dbg!(seen.len());
            return Some(source_est);
        }

        for next in curr.neighbors() {
            let s = source_est + 1;
            let t = next.target_estimate();
            let c = s + t;
            // if !seen.contains_key(&next) || c < seen[&next] {
            if let Some(&seen_c) = seen.get(&next)
                && c >= seen_c
            {
            } else {
                seen.insert(next.clone(), c);
                to_visit.push((Reverse(c), s, next));
            }
        }
    }

    None
}

impl State {
    fn num_objects(&self, floor: usize) -> u8 {
        let count = self.microchips[floor].count_ones() + self.generators[floor].count_ones();
        count as u8
    }

    /// This must be a lower bound on the distance to the target.
    fn target_estimate(&self) -> u8 {
        let mut i = 0;
        while i < NUM_FLOORS {
            if self.num_objects(i) != 0 {
                break;
            }
            i += 1;
        }
        let bottom_floor = i;

        let mut i = self.elevator as usize;
        let mut num_objects: [_; NUM_FLOORS] = array::from_fn(|i| self.num_objects(i));

        let mut total_cost = 0;
        while i > bottom_floor {
            assert_ne!(num_objects[i], 0);

            num_objects[i] -= 1;
            num_objects[i - 1] += 1;
            total_cost += 1;

            i -= 1;
        }

        let mut carried_objects = 0;
        for i in bottom_floor..NUM_FLOORS {
            carried_objects += num_objects[i];
            total_cost += if carried_objects >= 2 {
                carried_objects as u8 * 2 - 3
            } else {
                1
            };
        }
        total_cost
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        self.neighbors_including_invalid()
            .into_iter()
            .filter(Self::is_valid)
    }

    fn neighbors_including_invalid(&self) -> Vec<Self> {
        // can go up or down
        // can take any 1, or any 2 items with you

        let mut out = Vec::with_capacity(512);

        for dirn in [-1, 1] {
            let old_floor = usize::from(self.elevator);
            let new_floor = isize::try_from(old_floor).unwrap() + dirn;

            let in_bounds = 0 <= new_floor && new_floor < isize::try_from(NUM_FLOORS).unwrap();
            if !in_bounds {
                continue;
            }
            let new_floor = usize::try_from(new_floor).unwrap();

            // Note that x and y can be the same.
            // x = (i, a)
            for i in 0..2 {
                for a in 0..8 {
                    // y = (j, b)
                    for j in 0..2 {
                        for b in a..8 {
                            let x_le_y = (i, a) <= (j, b);
                            let x_exists = self.obj(i)[old_floor] & 1 << a != 0;
                            let y_exists = self.obj(j)[old_floor] & 1 << b != 0;

                            if x_le_y && x_exists && y_exists {
                                let mut new_state = self.clone();
                                new_state.elevator = u8::try_from(new_floor).unwrap();
                                new_state.obj_mut(i)[old_floor] &= !(1 << a);
                                new_state.obj_mut(j)[old_floor] &= !(1 << b);
                                new_state.obj_mut(i)[new_floor] |= 1 << a;
                                new_state.obj_mut(j)[new_floor] |= 1 << b;
                                out.push(new_state);
                            }
                        }
                    }
                }
            }
        }

        out
    }

    fn obj(&self, i: usize) -> &[u8; 4] {
        match i {
            0 => &self.microchips,
            1 => &self.generators,
            _ => panic!(),
        }
    }

    fn obj_mut(&mut self, i: usize) -> &mut [u8; 4] {
        match i {
            0 => &mut self.microchips,
            1 => &mut self.generators,
            _ => panic!(),
        }
    }

    fn is_valid(&self) -> bool {
        debug_assert!(self.elevator < u8::try_from(NUM_FLOORS).unwrap());
        (0..NUM_FLOORS).all(|i| {
            let exposed_microchips = self.microchips[i] & !self.generators[i];
            exposed_microchips == 0 || self.generators[i] == 0
        })
    }
}

/// Skip input parsing; hard-code instead.
impl State {
    fn start() -> Self {
        Self {
            elevator: 0,
            microchips: [0b1, 0, 0b_1111_0, 0],
            generators: [0b1, 0b_1111_0, 0, 0],
        }
    }

    fn target() -> Self {
        Self {
            elevator: 3,
            microchips: [0, 0, 0, 0b_1_1111],
            generators: [0, 0, 0, 0b_1_1111],
        }
    }

    fn start_part2() -> Self {
        Self {
            elevator: 0,
            microchips: [0b_11_0000_1, 0, 0b_1111_0, 0],
            generators: [0b_11_0000_1, 0b_1111_0, 0, 0],
        }
    }

    fn target_part2() -> Self {
        Self {
            elevator: 3,
            microchips: [0, 0, 0, 0b_11_1111_1],
            generators: [0, 0, 0, 0b_11_1111_1],
        }
    }
}

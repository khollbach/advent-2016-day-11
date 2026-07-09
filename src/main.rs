use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let ans = bfs(State::start(), State::target()).context("no path found")?;
    println!("{}", ans);

    let ans = bfs(State::start_part2(), State::target_part2()).context("no path found")?;
    println!("{}", ans);

    Ok(())
}

const NUM_FLOORS: isize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    elevator: isize,
    /// Sorted.
    elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Element {
    microchip: isize,
    generator: isize,
}

/// Find the shortest path from start to target.
fn bfs(start: State, target: State) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();

    seen.insert(start.clone());
    to_visit.push_back((start, 0));

    while let Some((curr, dist)) = to_visit.pop_front() {
        if curr == target {
            return Some(dist);
        }

        for next in curr.neighbors() {
            if !seen.contains(&next) {
                seen.insert(next.clone());
                to_visit.push_back((next, dist + 1));
            }
        }
    }

    None
}

impl State {
    fn neighbors(&self) -> Vec<Self> {
        self.neighbors_including_invalid()
            .into_iter()
            .filter(Self::is_valid)
            .collect()
    }

    fn neighbors_including_invalid(&self) -> Vec<Self> {
        // can go up or down
        // can take any 1, or any 2 items with you

        let mut out = vec![];

        for dirn in [-1, 1] {
            let old_floor = self.elevator;
            let new_floor = old_floor + dirn;

            let in_bounds = 0 <= new_floor && new_floor < NUM_FLOORS;
            if !in_bounds {
                continue;
            }

            #[derive(PartialEq, Eq)]
            enum ObjectType {
                Microchip,
                Generator,
            }

            let mut objects = vec![];
            for e in 0..self.elements.len() {
                if self.elements[e].microchip == old_floor {
                    objects.push((e, ObjectType::Microchip));
                }
                if self.elements[e].generator == old_floor {
                    objects.push((e, ObjectType::Generator));
                }
            }

            // Note that x and y can be the same.
            for x in &objects {
                for y in &objects {
                    let mut new_state = self.clone();
                    new_state.elevator = new_floor;
                    match x.1 {
                        ObjectType::Microchip => new_state.elements[x.0].microchip += dirn,
                        ObjectType::Generator => new_state.elements[x.0].generator += dirn,
                    }
                    if x != y {
                        match y.1 {
                            ObjectType::Microchip => new_state.elements[y.0].microchip += dirn,
                            ObjectType::Generator => new_state.elements[y.0].generator += dirn,
                        }
                    }
                    new_state.elements.sort();
                    out.push(new_state);
                }
            }
        }

        out
    }

    fn is_valid(&self) -> bool {
        assert!(self.elevator < NUM_FLOORS);
        self.elements.iter().all(|e| {
            let protected = e.microchip == e.generator;
            let fried = self.elements.iter().any(|e2| e2.generator == e.microchip);
            let safe = protected || !fried;
            safe
        })
    }
}

/// Skip input parsing; hard-code instead.
impl State {
    fn start() -> Self {
        Self {
            elevator: 0,
            // Note: this is sorted!
            elements: [
                // (microchip, generator)
                (0, 0), // promethium
                (2, 1), // cobalt
                (2, 1), // curium
                (2, 1), // ruthenium
                (2, 1), // plutonium
            ]
            .into_iter()
            .map(|(microchip, generator)| Element {
                microchip,
                generator,
            })
            .collect(),
        }
    }

    fn target() -> Self {
        Self {
            elevator: 3,
            elements: [
                (3, 3), // promethium
                (3, 3), // cobalt
                (3, 3), // curium
                (3, 3), // ruthenium
                (3, 3), // plutonium
            ]
            .into_iter()
            .map(|(microchip, generator)| Element {
                microchip,
                generator,
            })
            .collect(),
        }
    }

    fn start_part2() -> Self {
        Self {
            elevator: 0,
            elements: [
                (0, 0), // promethium
                (0, 0), // elerium
                (0, 0), // dilithium
                (2, 1), // cobalt
                (2, 1), // curium
                (2, 1), // ruthenium
                (2, 1), // plutonium
            ]
            .into_iter()
            .map(|(microchip, generator)| Element {
                microchip,
                generator,
            })
            .collect(),
        }
    }

    fn target_part2() -> Self {
        Self {
            elevator: 3,
            elements: [
                (3, 3), // promethium
                (3, 3), // elerium
                (3, 3), // dilithium
                (3, 3), // cobalt
                (3, 3), // curium
                (3, 3), // ruthenium
                (3, 3), // plutonium
            ]
            .into_iter()
            .map(|(microchip, generator)| Element {
                microchip,
                generator,
            })
            .collect(),
        }
    }
}

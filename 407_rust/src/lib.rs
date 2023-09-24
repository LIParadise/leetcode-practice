/// Given an m x n integer matrix heightMap representing the height of
/// each unit cell in a 2D elevation map, return the volume of water it can
/// trap after raining.
pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        #[derive(Clone, Copy)]
        enum DfsState {
            Undiscovered,
            Working,
            Done,
        }
        if height_map.len() == 0 || height_map[0].len() == 0 {
            return 0;
        }
        use DfsState::*;
        let mut water_map: Vec<Vec<(Option<i32>, DfsState)>> =
            vec![vec![(None, Undiscovered); height_map[0].len()]; height_map.len()];
        let mut max_wall = i32::MIN;
        height_map
            .iter()
            .zip(water_map.iter_mut())
            .for_each(|(row, d)| {
                *d.first_mut().unwrap() = (Some(-row.first().unwrap().clone()), Done);
                *d.last_mut().unwrap() = (Some(-row.last().unwrap().clone()), Done);
                if row.first().unwrap() > &max_wall {
                    max_wall = row.first().unwrap().clone();
                }
                if row.last().unwrap() > &max_wall {
                    max_wall = row.last().unwrap().clone();
                }
            });
        height_map
            .first()
            .unwrap()
            .iter()
            .zip(water_map.first_mut().unwrap().iter_mut())
            .for_each(|(col, d)| {
                *d = (Some(-*col), Done);
                if *col > max_wall {
                    max_wall = *col;
                }
            });
        height_map
            .last()
            .unwrap()
            .iter()
            .zip(water_map.last_mut().unwrap().iter_mut())
            .for_each(|(col, d)| {
                *d = (Some(-*col), Done);
                if *col > max_wall {
                    max_wall = *col;
                }
            });
        let max_wall = max_wall;
        let mut dfs_stack: Vec<(usize, usize)> = vec![];
        (1..height_map.len() - 1).for_each(|r| {
            (1..height_map[0].len() - 1).for_each(|c| {
                match water_map[r][c].0 {
                    Some(h) if h < 0 => {}
                    Some(h) if h >= 0 => {
                        let mut wall = max_wall;
                        let up = water_map.get(r - 1).map(|row| row.get(c));
                        let down = water_map.get(r + 1).map(|row| row.get(c));
                        let left = water_map.get(r).unwrap().get(c - 1);
                        let right = water_map.get(r).unwrap().get(c + 1);
                    }
                    Some(_) => panic!("Unexpected integer range"),
                    None => todo!(),
                };
            })
        });

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![10, 10, 9, 9, 9, 0],
                vec![10, 5, 4, 3, 2, 1],
                vec![10, 4, 3, 2, 1, 10],
                vec![10, 3, 2, 1, 0, 10],
                vec![10, 2, 1, 0, 0, 10],
                vec![10, 10, 10, 10, 10, 10]
            ]),
            9
        );
    }
}


#[derive(Debug)]
pub struct Blobservation {
    grid: Vec<Vec<u32>>,
}

impl Blobservation {
    pub fn new(r: &[Vec<u32>]) -> Self {
        Blobservation {grid: r.to_vec() }
    }
    pub fn read(&mut self, instructions: &str) {
        fn tilt_sequence(vec: Vec<u32>, len: usize) -> Vec<u32> {
            let result = vec.iter().fold(vec![vec![]], |mut v: Vec<Vec<u32>>, x| {
                match v.last_mut().unwrap().last_mut() {
                    Some(last) if x >= last => v.push(vec![*x]),
                    _ => v.last_mut().unwrap().push(*x),
                }
                v
            });
            let mut result = result
                .iter()
                .map(|x| x.iter().sum::<u32>())
                .collect::<Vec<u32>>();
            while result.len() != len {
                result.push(0);
            }
            result
        }
        fn tilt(vec: &mut Vec<Vec<u32>>, dir: char) {
            let rows = vec.len();
            let cols = vec[0].len();
            match dir {
                'N' => { (0..cols).for_each(|col| {
                    let temp = (0..rows)
                        .map(|row| vec[row][col])
                        .filter(|&x| x != 0)
                        .collect();
                    let temp = tilt_sequence(temp, cols);
                    (0..rows).for_each(|row| {
                        vec[row][col] = temp[row].clone();
                    }
                        );
                });
                    if vec.last().unwrap().iter().all(|&x| x == 0) {
                        vec.pop();
                    }
                },
                'S' => { (0..cols).for_each(|col| {
                    let temp = (0..rows)
                        .rev()
                        .map(|row| vec[row][col])
                        .filter(|&x| x != 0)
                        .collect();
                    let mut temp = tilt_sequence(temp, cols);
                    temp = temp.into_iter().rev().collect();
                    (0..rows).for_each(|row| vec[row][col] = temp[row].clone());
                });
                    if vec.first().unwrap().iter().all(|&x| x == 0) {
                        vec.remove(0);
                    }
                },
                'W' => { (0..rows).for_each(|row| {
                    let temp = (0..cols)
                        .map(|col| vec[row][col])
                        .filter(|&x| x != 0)
                        .collect();
                    let temp = tilt_sequence(temp, cols);
                    (0..cols).for_each(|col| vec[row][col] = temp[col].clone());
                });
                    if (0..rows).map(|r| vec[r].last().unwrap()).all(|&x| x == 0) {
                        (0..rows).for_each(|r| {vec[r].pop();});
                    };
                },
                'E' => { (0..rows).for_each(|row| {
                    let temp = (0..cols)
                        .rev()
                        .map(|col| vec[row][col])
                        .filter(|&x| x != 0)
                        .collect();
                    let mut temp = tilt_sequence(temp, cols);
                    temp = temp.into_iter().rev().collect();
                    (0..cols).for_each(|col| vec[row][col] = temp[col].clone());
                });
                    if (0..rows).map(|r| vec[r].first().unwrap()).all(|&x| x == 0) {
                        (0..rows).for_each(|r| {vec[r].remove(0);});
                    };
                },
                _ => panic!("WTF"),
            }
        }
        instructions.chars()
            .for_each(|card_dir| tilt(&mut self.grid, card_dir));
    }

    // you can change the `&self` parameter to a `&mut self` if suitable
    pub fn state(&self) -> Vec<Vec<u32>> {
        self.grid.clone()
    }
}


#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn example_test1(){
        // Verify correctness of step-by-step instructions
        let grid = vec![
            vec![9,4,6],
            vec![8,8,8],
            vec![3,6,9]
        ];
        let instructions = ["E","S","E","N"];
        let transition_steps = [
            vec![
                vec![0,9,10],
                vec![8,8,8],
                vec![0,0,18]
            ],
            vec![
                vec![0,9,10],
                vec![8,8,26]
            ],
            vec![
                vec![0,19],
                vec![8,34]
            ],
            vec![
                vec![8,19],
                vec![0,34]
            ]
        ];
        let mut blob = Blobservation::new(&grid);
        for (i,mov) in instructions.iter().enumerate() {
            blob.read(mov);
            assert_eq!(blob.state(),transition_steps[i]);
        }
    }

    #[test]
    fn example_test2(){
        // Test a simple case
        let grid = vec![
            vec![4,3,5],
            vec![1,4,6],
            vec![5,2,6]
        ];
        let final_state = vec![
            vec![7,0],
            vec![7,22]
        ];
        let mut blob = Blobservation::new(&grid);
        blob.read("WENS");
        assert_eq!(blob.state(),final_state);
    }
}
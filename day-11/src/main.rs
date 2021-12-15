extern crate utilities;

fn main() {
    let mut grid: Vec<Vec<i32>> = utilities::lines_from_file("input.txt")
        .unwrap()
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut count = 0;
    let mut iteration = 0;
    loop {
        grid.iter_mut().flatten().for_each(|x| *x += 1);
        loop {
            let mut run_again: bool = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] > 9 {
                        run_again = true;
                        count += 1;
                        grid[i][j] = -100;
                        for (x, y) in get_neighbours(i as i32, j as i32, grid.len() as i32) {
                            grid[x][y] += 1;
                        }
                    }
                }
            }
            if !run_again {
                break;
            }
        }
        grid.iter_mut().flatten().for_each(|x| { if *x < 0 { *x = 0 } });
        iteration += 1;
        if grid.iter().flatten().all(|x| *x == 0) {
            println!("Part two solution {}", iteration);
            break;
        }
        if iteration == 100 {
            println!("Part one solution {}", count);
        }
    }
}

fn get_neighbours(i: i32, j: i32, size: i32) -> Vec<(usize, usize)> {
    vec![
        (i + 1, j),
        (i - 1, j),
        (i + 1, j + 1),
        (i + 1, j - 1),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *x < size && *y >= 0 && *y < size)
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}

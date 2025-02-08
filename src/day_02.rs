use std::fs::read_to_string;

fn get_inputs() -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in read_to_string("./src/inputs/day_02.txt").unwrap().lines() {
        let mut report = Vec::new();
        for lvl in line.split_whitespace() {
            report.push(lvl.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    reports
}

pub fn part_1() {
    let reports = get_inputs();

    let mut safe_levels = 0;

    'outer: for rep in reports.iter() {
        let mut last_direction = 0; // 1 = increasing, -1 = decreasing
        for (i, pair_lvl) in rep.windows(2).enumerate() {
            let diff = pair_lvl[0] - pair_lvl[1];
            let new_direction;
            if diff == 0 || diff.abs() > 3 {
                continue 'outer;
            } else {
                if diff > 0 {
                    new_direction = 1;
                } else {
                    new_direction = -1;
                }
            }
            if i == 0 {
                last_direction = new_direction;
                continue;
            }
            if new_direction != last_direction {
                continue 'outer;
            }
            last_direction = new_direction;
        }
        safe_levels = safe_levels + 1;
    }

    println!("{safe_levels}");
}

pub fn part_2() {
    let reports = get_inputs();

    let mut safe_levels = 0;

    'outer: for rep in reports.iter() {
        let mut last_direction = 0;
        let still_safe = true;
        for (i, pair_lvl) in rep.windows(2).enumerate() {
            let diff = pair_lvl[0] - pair_lvl[1];
            let new_direction;
            if diff == 0 || diff.abs() > 3 {
                if still_safe {}
                continue 'outer;
            } else {
                if diff > 0 {
                    new_direction = 1;
                } else {
                    new_direction = -1;
                }
            }
            if i == 0 {
                last_direction = new_direction;
                continue;
            }
            if new_direction != last_direction {
                continue 'outer;
            }
            last_direction = new_direction;
        }
        safe_levels = safe_levels + 1;
    }

    println!("{safe_levels}");
}

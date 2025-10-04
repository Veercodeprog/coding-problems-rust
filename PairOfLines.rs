use std::io;

fn cross_product(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    a[0] * b[1] - a[1] * b[0]
}

fn check_if_all_points_lie_on_two_straight_line(points: Vec<Vec<i64>>) -> bool {
    let n = points.len();

    // Helper closure to check if all points in subset lie on a line defined by two points
    let all_collinear = |subset: &[Vec<i64>], a_idx: usize, b_idx: usize| -> bool {
        let ab = vec![
            subset[b_idx][0] - subset[a_idx][0],
            subset[b_idx][1] - subset[a_idx][1],
        ];
        for (i, p) in subset.iter().enumerate() {
            if i == a_idx || i == b_idx {
                continue;
            }
            let ap = vec![p[0] - subset[a_idx][0], p[1] - subset[a_idx][1]];
            if cross_product(&ab, &ap) != 0 {
                return false;
            }
        }
        true
    };

    // Try three candidate first lines using first 3 points
    let candidates = [(0, 1), (0, 2), (1, 2)];

    for &(i, j) in &candidates {
        let ab = vec![points[j][0] - points[i][0], points[j][1] - points[i][1]];

        let mut on_line = Vec::new();
        let mut off_line = Vec::new();

        for p in &points {
            let ap = vec![p[0] - points[i][0], p[1] - points[i][1]];
            if cross_product(&ab, &ap) == 0 {
                on_line.push(p.clone());
            } else {
                off_line.push(p.clone());
            }
        }

        if off_line.len() <= 1 {
            return true; // All remaining points lie on single point → YES
        }

        // Check if remaining points lie on second line
        if all_collinear(&off_line, 0, 1) {
            return true;
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    if n <= 3 {
        println!("YES");
        return;
    }

    let mut points: Vec<Vec<i64>> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        points.push(nums);
    }

    if check_if_all_points_lie_on_two_straight_line(points) {
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn calculate_fractal_dimension(heightmap: &Vec<Vec<f32>>) -> f32 {
    let mut sizes = Vec::new();
    let mut counts = Vec::new();

    let mut size = heightmap.len() as i32;

    while size > 1 {
        let count = count_boxes(heightmap, size);
        sizes.push(size);
        counts.push(count);
        size /= 2;
    }

    linear_regression(&sizes, &counts)
}

fn count_boxes(heightmap: &Vec<Vec<f32>>, size: i32) -> i32 {
    let mut count = 0;
    let step = size as usize;
    let h_len = heightmap.len();
    let w_len = heightmap[0].len();

    for i in (0..h_len).step_by(step) {
        for j in (0..w_len).step_by(step) {
            if check_box(heightmap, i, j, step) {
                count += 1;
            }
        }
    }

    count
}

fn check_box(heightmap: &Vec<Vec<f32>>, start_i: usize, start_j: usize, size: usize) -> bool {
    let mut covered = false;
    for i in start_i..std::cmp::min(start_i + size, heightmap.len()) {
        for j in start_j..std::cmp::min(start_j + size, heightmap[0].len()) {
            if heightmap[i][j] > 25.0 {
                covered = true;
                break;
            }
        }
        if covered {
            break;
        }
    }
    covered
}

fn linear_regression(x: &Vec<i32>, y: &Vec<i32>) -> f32 {
    let n = x.len() as f32;
    let (sum_x, sum_y, sum_xx, sum_xy) = x.iter().zip(y.iter()).fold(
        (0.0, 0.0, 0.0, 0.0),
        |(sx, sy, sxx, sxy), (&xi, &yi)| {
            let x = xi as f32;
            let y = yi as f32;
            let log_x = x.ln();
            let log_y = y.ln();
            (sx + log_x, sy + log_y, sxx + log_x * log_x, sxy + log_x * log_y)
        },
    );

    let denominator = n * sum_xx - sum_x * sum_x;
    let numerator = n * sum_xy - sum_x * sum_y;

    numerator / denominator
}
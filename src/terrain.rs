use crate::fft_utils::{apply_fft_to_grid, apply_ifft_to_grid, apply_pink_noise_filter, apply_low_pass_filter};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

// Generate heightmap using midpoint displacement
pub fn midpoint_displacement(n: u32, initial_roughness: f32, roughness_factor: f32, initial_max_height: f32) -> Vec<Vec<f32>> {
    let size: usize = 2_usize.pow(n) + 1;
    let mut heightmap = vec![vec![0.0; size]; size];

    let mut rng = rand::thread_rng();
    heightmap[0][0] = rng.gen_range(-initial_max_height..initial_max_height);
    heightmap[0][size - 1] = rng.gen_range(-initial_max_height..initial_max_height);
    heightmap[size - 1][0] = rng.gen_range(-initial_max_height..initial_max_height);
    heightmap[size - 1][size - 1] = rng.gen_range(-initial_max_height..initial_max_height);

    let mut gap_size: usize = size - 1;
    let mut current_range = initial_max_height;
    let mut roughness = initial_roughness;

    while gap_size > 1 {

        //square step
        for i in ((gap_size / 2)..(size)).step_by(gap_size) {
            for j in ((gap_size / 2)..(size)).step_by(gap_size) {
                let midpoint = (heightmap[i - gap_size / 2][j - gap_size / 2] + heightmap[i + gap_size / 2][j - gap_size / 2] + heightmap[i - gap_size / 2][j + gap_size / 2] + heightmap[i + gap_size / 2][j + gap_size / 2]) / 4.0;
                heightmap[i][j] = midpoint + rng.gen_range(-current_range..=current_range) * roughness;
            }
        }

        //diamond step
        let half_gap = gap_size / 2;
        for i in (0..(size)).step_by(half_gap) {
            for j in ((i + half_gap) % gap_size..size).step_by(gap_size) {

                let mut sum_of_corners: f32 = 0.0;
                let mut amount_of_corners: f32 = 0.0;

                if j > half_gap {
                    sum_of_corners += heightmap[i][j - half_gap];
                    amount_of_corners += 1.0;
                }
                if j + half_gap < size {
                    sum_of_corners += heightmap[i][j + half_gap];
                    amount_of_corners += 1.0;
                }
                if i >= half_gap {
                    sum_of_corners += heightmap[i - half_gap][j];
                    amount_of_corners += 1.0;
                }
                if i + half_gap < size {
                    sum_of_corners += heightmap[i + half_gap][j];
                    amount_of_corners += 1.0;
                }

                let average = sum_of_corners / amount_of_corners;
                heightmap[i][j] = average + rng.gen_range(-current_range..=current_range) * roughness;
            }
        }

        gap_size = gap_size / 2;
        roughness = roughness * roughness_factor;
    }

    return heightmap;
}


pub fn fft_terrain(n: u32) -> Vec<Vec<f32>> {
    let size = 2usize.pow(n);
    let mut rng = thread_rng();

    // Initializing and populating the heightmap with random values
    // It initializes them as complex numbers because it is easier to 
    // manage them later on as FFT operates over them. The real
    // part of the complex value is populated and the imaginary is set
    // to zero initially.
    let mut heightmap: Vec<Vec<Complex<f32>>> = (0..size).map(|_|
        (0..size).map(|_|
            Complex::new(rng.gen_range(-1.0..1.0), 0.0)
        ).collect()
    ).collect();

    apply_fft_to_grid(&mut heightmap, size);
    apply_pink_noise_filter(&mut heightmap, size);
    apply_ifft_to_grid(&mut heightmap, size);

    let vertical_scale = (size as f32) * 2.0;
    for row in &mut heightmap {
        for height in row.iter_mut() {
            *height *= vertical_scale;
        }
    }

    // Converting complex results back to real values
    heightmap.into_iter()
        .map(|row| row.into_iter().map(|c| c.re.abs()).collect())
        .collect()

}

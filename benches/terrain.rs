//use crate::fft_utils::{apply_fft_to_grid, apply_ifft_to_grid, apply_pink_noise_filter, apply_low_pass_filter};
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use std::sync::Arc; 

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

    let vertical_scale = 500.0;
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

//Applies fft to a vector
pub fn apply_fft_to_vec(vec: &mut Vec<Complex<f32>>, fft: &Arc<dyn rustfft::Fft<f32>>) {
    fft.process(vec);
}

//Applies fft to a grid by applying fft row wise and column wise
pub fn apply_fft_to_grid(heightmap: &mut Vec<Vec<Complex<f32>>>, size: usize) {

    let mut planner: FftPlanner<f32> = FftPlanner::new();
    let fft = planner.plan_fft_forward(size);
    
    // Applying FFT and filter row-wise
    for row in heightmap.iter_mut() {
        apply_fft_to_vec(row, &fft);
    }

    // Transposing so that we can apply FFT to columns
    let mut transposed = vec![vec![Complex::new(0.0, 0.0); size]; size];
    for i in 0..size {
        for j in 0..size {
            transposed[j][i] = heightmap[i][j];
        }
    }

    // Applying FFT column-wise
    for col in transposed.iter_mut() {
        apply_fft_to_vec(col, &fft);
    }

    // Transposing back
    for i in 0..size {
        for j in 0..size {
            heightmap[j][i] = transposed[i][j];
        }
    }
}

//Applies ifft by applying ifft row wise and column wise with a scaling factor
pub fn apply_ifft_to_grid(heightmap: &mut Vec<Vec<Complex<f32>>>, size: usize) {

    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(size);

    // Applying ifft row-wise
    for row in heightmap.iter_mut() {
        apply_fft_to_vec(row, &ifft);
    }

    // Apply scaling factor after IFFT
    let scale_factor = 1.0 / (size) as f32; // Calculate the scaling factor

    for row in heightmap.iter_mut() {
        for value in row.iter_mut() {
            *value = *value * scale_factor;
        }
    }

    let mut transposed = vec![vec![Complex::new(0.0, 0.0); size]; size];
    for i in 0..size {
        for j in 0..size {
            transposed[j][i] = heightmap[i][j];
        }
    }

    // Applying ifft column-wise
    for col in transposed.iter_mut() {
        apply_fft_to_vec(col, &ifft);
    }

    // Apply scaling factor after IFFT
    for row in transposed.iter_mut() {
        for value in row.iter_mut() {
            *value = *value * scale_factor;
        }
    }

    for i in 0..size {
        for j in 0..size {
            heightmap[j][i] = transposed[i][j];
        }
    }
}

//Pink noise filter function. Pink noise filter provided better results than other attempted
//filters.
pub fn apply_pink_noise_filter(heightmap: &mut Vec<Vec<Complex<f32>>>, size: usize) {
    let center = size as f32 / 2.0;

    for i in 0..size {
        for j in 0..size {
            let distance = (((i as f32 - center).powi(2) + (j as f32 - center).powi(2)).sqrt()).abs().max(1.0); 
            let attenuation = (1.0) / (distance.powf(2.0) * 0.15); 
            
            heightmap[i][j] *= attenuation;
        }
    }
}

pub fn apply_low_pass_filter(heightmap: &mut Vec<Vec<Complex<f32>>>, size: usize) {
   let center = size as f32 / 2.0;

    for i in 0..size {
        for j in 0..size {
            let distance = (((i as f32 - center).powi(2) + (j as f32 - center).powi(2)).sqrt()).max(1.0);
            let attenuation = if distance < 3.5 {
                Complex::new(1.0, 1.0)
            } else {
                Complex::new(0.0, 0.0)
            };  
            
            println!("{:?}", attenuation);

            heightmap[i][j] *= attenuation;
        }
    }
}

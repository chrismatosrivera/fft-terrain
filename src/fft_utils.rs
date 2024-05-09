use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
extern crate rand;
use rand::thread_rng;
use rand::Rng;
use std::sync::Arc; 

//Applies fft to a vector
pub fn apply_fft_to_vec(vec: &mut Vec<Complex<f32>>, fft: &Arc<dyn rustfft::Fft<f32>>) {
    fft.process(vec);
} 

//NOTE: This is a local implementation of FFT in Rust. It is not used for the purpose
//of this project due to a noticeable difference between this and the highly optimized
//RustFFT crate.
pub fn apply_fft(vec: &mut Vec<Complex<f32>>) {
    let n = vec.len();

    // Rearrange the elements in the vector by bit-reversed indices
    let mut i = 0;
    for j in 1..n {
        let mut bit = n >> 1;
        while i & bit != 0 {
            i ^= bit;
            bit >>= 1;
        }
        i ^= bit;
        if i > j {
            vec.swap(i, j);
        }
    }

    for s in 1..=(n as f32).log2() as usize {
        let m = 1 << s;
        let w_m = Complex::from_polar(1.0, 2.0 * std::f32::consts::PI / m as f32);
        for k in (0..n).step_by(m) {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..(m/2) {
                let t = w * vec[k + j + m/2];
                let u = vec[k + j];
                vec[k + j] = u + t;
                vec[k + j + m/2] = u - t;
                w = w * w_m;
            }
        }
    }
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

            heightmap[i][j] *= attenuation;
        }
    }
}

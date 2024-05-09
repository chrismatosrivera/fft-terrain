use noise::{NoiseFn, Perlin, Simplex, Worley};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

pub enum NoiseType {
    Perlin,
    Worley,
    Simplex
}

// Define a trait for noise generation
trait NoiseGenerator {
    fn generate_noise(&self, point: [f64; 2]) -> f64;
}

// Implement the trait for Perlin noise
impl NoiseGenerator for Perlin {
    fn generate_noise(&self, point: [f64; 2]) -> f64 {
        // Assume Perlin already has a method `get`
        self.get(point)
    }
}

// Implement the trait for Simplex noise
impl NoiseGenerator for Simplex {
    fn generate_noise(&self, point: [f64; 2]) -> f64 {
        // Similarly, assume Simplex has a `get` method
        self.get(point)
    }
}

// Implement the trait for Worley noise
impl NoiseGenerator for Worley {
    fn generate_noise(&self, point: [f64; 2]) -> f64 {
        // Assuming Worley also has a `get` method
        self.get(point)
    }
}

pub fn noise_terrain(n: u32, noise_type: NoiseType) -> Vec<Vec<f32>> {
    let mut rng = thread_rng();
    let size = 2usize.pow(n);

    let noise_generator: Box<dyn NoiseGenerator> = match noise_type {
        NoiseType::Perlin => Box::new(Perlin::new(rng.gen_range(0..100))),
        NoiseType::Simplex => Box::new(Simplex::new(rng.gen_range(0..100))),
        NoiseType::Worley => Box::new(Worley::new(rng.gen_range(0..100))),
    };

    let mut noise_grid = Vec::with_capacity(size);

    // These parameters can be adjusted to change the noise characteristics.
    let base_scale = 0.005; // Smaller scale for larger terrains
    let octaves = 2; // More octaves for additional complexity
    let persistence = 0.5;
    let lacunarity = 2.0;

    let mut max_value = f32::MIN;
    let mut min_value = f32::MAX;

    for y in 0..size {
        let mut row = Vec::with_capacity(size);
        for x in 0..size {
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut noise_value = 0.0;

            // Generate noise with multiple octaves
            for _ in 0..octaves {
                noise_value += noise_generator.generate_noise([
                    x as f64 * base_scale * frequency, 
                    y as f64 * base_scale * frequency,
                ]) as f32 * amplitude;

                amplitude *= persistence;
                frequency *= lacunarity;
            }

            max_value = max_value.max(noise_value);
            min_value = min_value.min(noise_value);

            row.push(noise_value);
        }
        noise_grid.push(row);
    }

    // Normalize and apply a curve to emphasize elevation changes
    for y in 0..size {
        for x in 0..size {
            let normalized_value = (noise_grid[y][x] - min_value) / (max_value - min_value);
            // Apply an exponential curve to make the terrain more varied.
            noise_grid[y][x] = normalized_value * 50.0;
        }
    }

    noise_grid
}

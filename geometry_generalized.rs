// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

// Here we generalize to include vectors of differing lengths.
// We generalize by using slice references, where size
// does not need to be declared for the reference.
// But efficiency takes a hit due to checking length at runtime.

fn magnitude(v: &[f64]) -> f64 {
    let mut curr_mag: f64 = 0.0;
    for coord in v {
        curr_mag += coord.powi(2);
    }
    return curr_mag.sqrt();
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
// Use * to dereference and change the value of the original vector.


fn normalize(v: &mut [f64]) {
    let mag = magnitude(v);
    for coord in v {
        *coord /= mag;
    }
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0, 3.0, 4.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

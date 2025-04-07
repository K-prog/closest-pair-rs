use closest_pair_rs::utils::*;
use closest_pair_rs::algorithms::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = [1000, 10000, 100000, 1000000, 5000000, 7000000, 10000000])]
fn test_closest_pair_bit_shift(n: u32) -> (Point, Point, f32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bits = 31; 
    
    // Generate 100 random points
    let mut points = Vec::new();
    for _ in 0..n {
        points.push(Point {
            x: rng.gen_range(0..(u32::pow(2, bits))),
            y: rng.gen_range(0..(u32::pow(2, bits)))
        });
    }
    
    // Run closest pair algorithm
    let (p1, p2, dist) = closest_pair_bit_shift(points.clone(), bits as u8);
    
    // // Compare with brute force result for validation
    // let (bf_p1, bf_p2, bf_dist) = closest_pair_brute_force(points);
    
    // // The results should match
    // assert!((dist - bf_dist).abs() < 0.001);
    
    // // Check that the points match (in either order)
    // assert!(
    //     (p1.x == bf_p1.x && p1.y == bf_p1.y && p2.x == bf_p2.x && p2.y == bf_p2.y) ||
    //     (p1.x == bf_p2.x && p1.y == bf_p2.y && p2.x == bf_p1.x && p2.y == bf_p1.y)
    // );

    (p1, p2, dist)
}

#[divan::bench(args = [1000, 10000, 100000, 1000000, 5000000, 7000000, 10000000])]
fn test_closest_pair_optimized(n: u32) -> (Point, Point, f32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bits = 31; 
    
    // Generate 100 random points
    let mut points = Vec::new();
    for _ in 0..n {
        points.push(Point {
            x: rng.gen_range(0..(u32::pow(2, bits))),
            y: rng.gen_range(0..(u32::pow(2, bits)))
        });
    }
    
    // Run closest pair algorithm
    let (p1, p2, dist) = closest_pair_optimized(points.clone());
    
    // // Compare with brute force result for validation
    // let (bf_p1, bf_p2, bf_dist) = closest_pair_brute_force(points);
    
    // // The results should match
    // assert!((dist - bf_dist).abs() < 0.001);
    
    // // Check that the points match (in either order)
    // assert!(
    //     (p1.x == bf_p1.x && p1.y == bf_p1.y && p2.x == bf_p2.x && p2.y == bf_p2.y) ||
    //     (p1.x == bf_p2.x && p1.y == bf_p2.y && p2.x == bf_p1.x && p2.y == bf_p1.y)
    // );

    (p1, p2, dist)
}
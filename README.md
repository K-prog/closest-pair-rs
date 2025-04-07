
# Closest Pair Algorithm: A Faster and Comprehensible Approach Using Bit Shift
![Codecov](https://img.shields.io/codecov/c/github/K-prog/closest-pair-rs)

An elegant yet comprehensible solution for the Closest Pair Problem in a 2D plane, deemed to be faster than the current best implementation.

## Idea
To pack each 2D point in a given set _A_ using bit shifting techniques to reduce the spatial dimension of _A_ and then perform the search.
```rust 
/// Packs two positive numbers into a single number using bit shifting
pub fn pack_numbers(num1: u32, num2: u32, bits: u8) -> u64 {
    let mask = (1u64 << bits) - 1;
    ((num1 as u64 & mask) << bits) | (num2 as u64 & mask)
}
```
_Note: The current implementation offers you to dynamically allocate bits consumed for both points(better for sets with low maximum range)_

Consider, 
```
Array A = {(xᵢ, yᵢ) ∈ [0 ,2³²-1]},  where A yields 

Array A' = {aᵢ ∈ [0 ,2⁶⁴-1]}, such that aᵢ = (xᵢ << 32)| yᵢ (<< denotes left shift operation, | bitwise OR)
```
\
We now apply any sorting technique(can use unstable sort as ordering does not matter) on _A’_, and intuitively search the closest pair in the adjacent indices _∀aᵢ_, upon experimentation, we find packing introduces discontinuities where close points end up far apart in the _A’_.

The probability of finding the closest pair increases as you increase the search window size and becomes 100% when the window size is till the number of bits shifted.
```
∀aᵢ search space ∈ [aᵢ+1, aᵢ+32]
```
![App Screenshot](https://miro.medium.com/v2/resize:fit:4800/format:webp/1*xPKFV7XSN6m7YnFGLrz7_Q.png)

## Benchmark
benchmark in your local machine
```bash
cargo bench
```
My system has around 4.45x speedup

![App Screenshot](https://miro.medium.com/v2/resize:fit:4800/format:webp/1*SbqSbzGpMFoXGEyUIGePUw.png)

## Output validation
Test cases for validation are implemented and can be adjusted accordingly
```bash
cargo test
```

## Time Complexity
```
nlog(n)+(bits*n) -> O(nlogn)
```
_Includes: Sorting of the packed array, checking in the window size limited by the bit shift number_

## Limitations

1) The algorithm just works with positive points, i.e., any set of points that lie in the 1st Quadrant of the 2D Cartesian Plane. A workaround would be to process all points to fall in the 1st quadrant by adding some constant _c_ to both x and y coordinates.

2) The algorithm works by combining x and y values in bits, so you can only process numbers that are within half the number of max bits supported by your compiler.

3) The algorithm does not work with floating point values(for now).

#### Note: The proof behind how the closest pair lies within the window of number of bits shifted _∀aᵢ_ is yet to be discovered ;-; 

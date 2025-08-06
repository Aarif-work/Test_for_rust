fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum as f32 / count as f32;

    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
    println!("Maximum: {}", max);
    println!("Minimum: {}", min);
}
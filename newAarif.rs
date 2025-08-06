fn main(){
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");
    println!("Length: {}, Capacity: {}", fruits.len(), fruits.capacity());
}
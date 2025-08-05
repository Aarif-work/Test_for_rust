fn memory() {
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");

    println!("Length: {}, Capacity: {}", fruits.len(), fruits.capacity());
        println!("Start memory address (heap buffer): {:p}", fruits.as_ptr());
          fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
     
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
  println!("Length: {}, Capacity: {}", fruits.len(), fruits.capacity());
        println!("Start memory address (heap buffer): {:p}", fruits.as_ptr());
  }
 // changed ->
  
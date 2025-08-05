fn main() {
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");
    //println!("{:?}", fruits);
    println!("Length: {}, Capacity: {}", fruits.len(), fruits.capacity());
        println!("Start memory address (heap buffer): {:p}", fruits.as_ptr());
          fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
      fruits.insert(0, "apple");
    fruits.insert(0, "apple");
    fruits.insert(0, "apple");
  println!("{:?}", fruits);
  println!("{:?}", fruits);
  println!("Length: {}, Capacity: {}", fruits.len(), fruits.capacity());
        println!("Start memory address (heap buffer): {:p}", fruits.as_ptr());
  }
  // println!("Length: {}, Capacity: {}", v.len(), v.capacity());
  // v.push(8);
  // println!("Length: {}, Capacity: {}", v.len(), v.capacity());
  
  // }
  
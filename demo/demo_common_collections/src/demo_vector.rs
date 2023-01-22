pub fn demo_vectors() {
    let a = [1, 2, 3];

    let mut v1: Vec<i32> = Vec::new(); // Here we have to specify the type.
    v1.push(1);
    v1.push(2);
    v1.push(3);

    {
        let v2 = vec![1, 2, 3]; // Here Rust can infer the type.
    } // Vectors are stored on the heap. When the scope ends, v2 and all the elements inside it will be dropped
}

pub fn demo_element_accessing() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    // let third = v[2]; // ??? What's the difference here
    v.push(6);

    // !!! The "third" below is the same one above, thus "v.push" will cause error
    // println!("The third element is {}", third);

    match v.get(2) {
        // !!! The "third" below is not the same one above.
        // It comes from the return value of the v.get(2) if it is Some(_).
        Some(third) => println!("The third element is {}", third),
        None => println!("The index is out of bounds"),
    }
}

pub fn demo_iterating_elements() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Take immutable reference
    for i in &v {
        println!("{}", i);
    }

    // Take mutable reference
    for i in &mut v {
        *i += 50; // ??? We need to dereference here
        println!("{}", i);
    }
}

pub fn demo_store_enum_into_vec() {
    // Use the vector to store different types of data, i.e. a row of cells in a spreadsheet.
    // Each cell could store either an integer a floating point number or a string.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(89.64),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => print!("{}", i),
        _ => println!("Not a integer!"),
    }
}

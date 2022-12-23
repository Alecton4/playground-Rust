// REF: https://www.youtube.com/watch?v=6rcTSxPJ6Bw

fn main() {
    let number_list = vec![19, 89, 64, 100, 30];
    println!("The largest number is {}", get_largest(number_list));

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    println!("The largest char is {}", get_largest_generic(char_list));

    let p1 = Point { x: 89, y: 64 };
    println!("{}", p1.get_x());
    let p2 = Point { x: 89.0, y: 64.0 };
    println!("{}, {}", p2.get_x(), p2.get_y());

    let p3 = Point { x: 89, y: 64 };
    let p4 = Point {
        x: "八九",
        y: "六四",
    };
    let p5 = p3.mixup(p4);
    println!("{}", p5.get_x());
}

// This function can only handle i32 vector
fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// This func can handle all types of vectors which have "PartialOrd + Copy" traits
fn get_largest_generic<Type: PartialOrd + Copy>(list: Vec<Type>) -> Type {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<X, Y> Point<X, Y> {
    // NOTE: self vs &self
    fn get_x(&self) -> &X {
        &self.x
    }
}

impl Point<f64, f64> {
    // NOTE: get_y is only implemented for f64 points
    fn get_y(&self) -> f64 {
        self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, U> {
        Point {
            x: other.x,
            y: self.y,
        }
    }
}

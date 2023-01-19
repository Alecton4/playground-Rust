// REF: https://www.youtube.com/watch?v=kZXJvLfjUS4
// TODO: Review
// Closure is like function but it's anonymous.
// Closure can be stored as variables and passed around.
// Closure can be passed as input parameter to a function.
// Closure has access to variables that are defined within the scope in which the closure is defined.

use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 8;
    generate_workout_using_closure(simulated_intensity, simulated_random_number);

    demo_cap_env();
}

// Suppose building the backend for a fitness app.
// The app will generate customized workouts for a user based on their profile.
// Suppose part of this algorithm runs a calculation that is expensive and takes a few seconds to run.
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// We want to refactor the below code so that the expensive function is only executed when necessary.
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else if random_number == 3 {
        println!("Today, take a break!");
    } else {
        println!(
            "Today, run for {} minutes!",
            simulated_expensive_calculation(intensity)
        );
    }
}

// Create a struct holding the closure and the result of the closure
struct Cache<T>
where
    // NOTE: All closures and functions implement one of the three traits:
    // Fn: Immutably borrow values.
    // FnMut: Mutably borrow values.
    // FnOnce: Take ownership of the variables inside the closure environment.
    // TODO: review
    // Fn trait has FnMut as a supertrait,
    // and FnMut has FnOnce as a supertrait.
    // That means that Fn has all functionalities of Fn+FnMut+FnOnce,
    // FnMut has functionalities of FnMut+FnOnce,
    // and FnOnce only has FnOnce.
    // Fn can do immutable, mutable, and capture.
    // FnMut can only do mutable and capture.
    // And FnOnce can only capture.
    T: Fn(u32) -> u32,
{
    expensive_calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(expensive_calculation: T) -> Cache<T> {
        Cache {
            expensive_calculation,
            value: None,
        }
    }

    // The value is calculated only once.
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.expensive_calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_using_closure(intensity: u32, random_number: u32) {
    // We can omit type annotations.
    let mut cached_result = Cache::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // The following is also doable:
    // let mut cached_result = Cacher::new(simulated_expensive_calculation);

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else if random_number == 3 {
        println!("Today, take a break!");
    } else {
        println!("Today, run for {} minutes!", cached_result.value(intensity));
    }
}

// Capture the environment with closure
fn demo_cap_env() {
    let x = vec![1, 2, 3];

    // "x" is defined outside the closure but can be accessed
    // because "x" and this closure are defined within the same scope.
    let equal_to_x = |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    // NOTE: The following closure will take ownership of "x".
    // Both this one and the above one will take ownership of "y".
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

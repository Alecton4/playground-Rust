mod greet;
mod guess;

fn main() {
    greet::greet_world();
    guess::guess_hello();
    guess::guess_number();
}

// sort the array in descending order
fn sort_array(arr: &mut Vec<i32>) -> Vec<i32> {
    arr.sort_by(|a, b| b.cmp(a));
    arr.to_vec()
}

// search substrings in a string ignoring case
fn contain_substring(src: &str, sub: &str) -> bool {
    src.to_lowercase().contains(sub.to_lowercase().as_str())
}

// count substrings in a string ignoring case
fn count_substring(src: &str, sub: &str) -> i32 {
    src.to_lowercase()
        .matches(sub.to_lowercase().as_str())
        .count() as i32
}

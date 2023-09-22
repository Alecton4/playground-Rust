use rand::{distributions::Uniform, Rng};
use std::io;

fn main() {
    test_quick_sort();
    test_get_two();
}

fn randomized_partition(arr: &mut [i32], idx_start: usize, idx_end: usize) -> usize {
    // Randomize
    let idx_randomized_pivot = rand::thread_rng().sample(Uniform::from(idx_start..=idx_end));
    arr.swap(idx_randomized_pivot, idx_end);

    // Start partitioning
    let mut idx_pivot = idx_start;
    let pivot = arr[idx_end];
    for idx_curr in idx_start..=idx_end {
        if arr[idx_curr] <= pivot {
            // Swap current and pivot(position)
            arr.swap(idx_curr, idx_pivot);
            // Update idx_pivot
            idx_pivot += 1;
        }
    }
    idx_pivot - 1 // Note the -1 here
}

fn quick_sort(arr: &mut [i32], idx_start: usize, idx_end: usize) {
    // if idx_start >= idx_end {
    //     return;
    // }

    let idx_pivot = randomized_partition(arr, idx_start, idx_end);
    if idx_pivot > idx_start {
        // usize cannot be a negative value
        quick_sort(arr, idx_start, idx_pivot - 1);
    }
    if idx_pivot < idx_end {
        quick_sort(arr, idx_pivot + 1, idx_end);
    }
}

fn test_quick_sort() {
    // Get array length
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Cannot read line!");
    let arr_len: usize = input_line.trim().parse().expect("Not an integer!");

    // Generate array
    let mut arr: Vec<i32> = rand::thread_rng()
        .sample_iter(Uniform::from(1..=100))
        .take(arr_len)
        .collect(); // REF: generate random vector: https://stackoverflow.com/a/48219147
    println!("{:?}", arr);

    // Sort array
    quick_sort(&mut arr, 0, arr_len - 1);
    println!("{:?}", arr);
}

// REF: https://discord.com/channels/442252698964721669/448238009733742612/1055720156883595284
// There are a few methods that all use unsafe code with pointer math internally.
// For example, the function iter_mut gives you an iterator over mutable references to different elements of the slice.
// Saving the two of those that you care about gives you the references you need.
// Another common way is split_at_mut,
// which splits your slice at an index to give you two smaller slices that you can index with no problem.

// REF: https://discord.com/channels/442252698964721669/448238009733742612/1055721340335824907
// An example using iterators (I wrote this one and it’s a little more limited because the indices must be in order)
fn get_two_1<T>(v: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    let diff = j.checked_sub(i)?.checked_sub(1)?;
    let mut iter = v.iter_mut().skip(i);
    Some((iter.next()?, iter.nth(diff)?))
}

// An example using split at mut (credit to orlp from the community server)
fn get_two_2<T>(v: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i < j && j < v.len() {
        let (a, b) = v.split_at_mut(j);
        Some((&mut a[i], &mut b[0]))
    } else if j < i && i < v.len() {
        let (a, b) = v.split_at_mut(i);
        Some((&mut b[0], &mut a[j]))
    } else {
        None
    }
}

fn test_get_two() {
    let mut items = [0, 1, 2, 3, 4];
    assert_eq!(get_two_1(&mut items, 1, 2), Some((&mut 1, &mut 2)));
    assert_eq!(get_two_1(&mut items, 0, 4), Some((&mut 0, &mut 4)));
    // out of order is no good
    assert_eq!(get_two_1(&mut items, 4, 0), None);
    // out of bounds is no good
    assert_eq!(get_two_1(&mut items, 4, 6), None);
}

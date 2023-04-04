use rand::Rng;

mod day1;

use day1::linear_search_list::linear_search;
use day1::binary_search_list::binary_search;
use day1::two_crystal_balls::two_crystal_balls;
use day1::bubble_sort::bubble_sort;

fn main() {
    println!("Just run `cargo test` bro");
}

///
/// Testing bro
///
#[test]
fn linear_search_test() {
    let owo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(linear_search(&owo, &69), true);
    assert_eq!(linear_search(&owo, &1336), false);
    assert_eq!(linear_search(&owo, &69420), true);
    assert_eq!(linear_search(&owo, &69421), false);
    assert_eq!(linear_search(&owo, &1), true);
    assert_eq!(linear_search(&owo, &0), false);
}

#[test]
fn binary_search_test() {
    let owo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(binary_search(&owo, 69), true);
    assert_eq!(binary_search(&owo, 1336), false);
    assert_eq!(binary_search(&owo, 69420), true);
    assert_eq!(binary_search(&owo, 69421), false);
    assert_eq!(binary_search(&owo, 1), true);
    assert_eq!(binary_search(&owo, 0), false);
}

#[test]
fn two_crystal_balls_test() {
    let mut bool_array = [false; 10000];

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..bool_array.len());
    for i in idx..bool_array.len() {
        bool_array[i] = true;
    }

    assert_eq!(two_crystal_balls(&bool_array), Some(idx));

    for i in 0..bool_array.len() {
        bool_array[i] = false;
    }
    assert_eq!(two_crystal_balls(&bool_array), None);
}

#[test]
fn bubble_sort_test() {
    let mut input = [9, 3, 7, 4, 69, 420, 42];

    let result = [3, 4, 7, 9, 42, 69, 420];

    bubble_sort(&mut input);

    assert_eq!(input, result);
}


mod day1;

use day1::linear_search_list::linear_search;
use day1::binary_search_list::binary_search;

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


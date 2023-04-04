mod day1;

use day1::linear_search_list::linear_search;

fn main() {
    // let elements = [ 100, 99, 11, 8 ];
    // let mut test_case = 1;
    // let result = linear_search(&elements, test_case);

    // println!("Result for {} is: {} -> should be false", test_case, result);
    // 
    // test_case = 11;
    // let result = linear_search(&elements, test_case);

    // println!("Result for {} is: {} -> should be true", test_case, result);
}

///
/// Testing bro
///
#[test]
fn uwu() {
    let owo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(linear_search(&owo, 69), true);
    assert_eq!(linear_search(&owo, 1336), false);
    assert_eq!(linear_search(&owo, 69420), true);
    assert_eq!(linear_search(&owo, 69421), false);
    assert_eq!(linear_search(&owo, 1), true);
    assert_eq!(linear_search(&owo, 0), false);
}

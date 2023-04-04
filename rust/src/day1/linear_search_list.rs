
// BigO: O(n)
pub fn linear_search(haystack: &[i32], needle: &i32) -> bool {
    for item in haystack.iter() {
        if item == needle {
            return true;
        }
    }
    false
}

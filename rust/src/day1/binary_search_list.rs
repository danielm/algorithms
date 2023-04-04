
// BigO: O(logN)
/*
* When performing +1/-1 checks try to follow a conversion like: [low, high)
* That means: low is always inclusive, high is always exclusive
*
*/
pub fn binary_search(haystack: &[usize], needle: usize) -> bool {
    let mut low = 0.0;
    let mut high = haystack.len() as f32;

    while low < high {
        let middle: usize = f32::floor(low + (high - low) / 2.0) as usize;
        let current = haystack[middle];

        if current == needle {
            return true;
        } else if current > needle { // 81 > 69
            high = middle as f32;
        } else {
            low = middle as f32 + 1.0;// +1 because we know middle isnt the answer
        }
    } 

    false
}

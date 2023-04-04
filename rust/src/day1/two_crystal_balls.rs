
// BigO: O(sqrt(N))

pub fn two_crystal_balls(haystack: &[bool]) -> Option<usize> {
    // Note: We asume the array is long enougth to have an sqrt: (1 for 3)
    let jump = (haystack.len() as f64).sqrt().floor() as usize;

    let mut i = jump;

    // Find first time is true (where is break)--> this doesnt garantees is the first, so we have
    // to jump back first (worst case)
    while i < haystack.len() {
        if haystack[i] {
            break;
        }
        i += jump;
    }

    // Now we jump back, and walk forward (linear search ackshhhualy)
    let end = i;
    i -= jump;

    while i < end {
        if haystack[i] {
            return Some(i);
        }
        i += 1;
    }

    None
}

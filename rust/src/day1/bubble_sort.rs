
// BigO: O(n^2) 

pub fn bubble_sort(haystack: &mut [usize]) {
    let len = haystack.len();

    /*
    * So based on the coment below, these two loops are the ones that cause squared algorithm,
    * because we ar going over the length of the array, en every time we go over we go over again
    * some or most of the array everysinge time
    */
    for i in 0..len {
        for j in 0..len-1-i {
            let next = j+1;

            /*
            * The runtime of accessin an alement of an array is O(1) -> Fixed computational cost
            * If we access it twice, like below, its just 2 Constant
            * So this is all constant operations, all here is constant
            * Nothing in here is based on input size.
            */
            if haystack[j] > haystack[next] {
                haystack.swap(j, next)
            }
        }
    }
}

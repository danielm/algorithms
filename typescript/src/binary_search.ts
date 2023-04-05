// BigO: O(logN)
/*
* When performing +1/-1 checks try to follow a conversion like: [low, high)
* That means: low is always inclusive, high is always exclusive
*
*/
export function binary_search(haystack: number[], needle: number): boolean {

  let low = 0;
  let high = haystack.length;

  while (low < high) {
    let middle = Math.floor(low + (high - low) / 2);
    let current = haystack[middle];

    if (current == needle) {
      return true;
    } else if (current > needle) {
      high = middle;
    } else {
      low = middle + 1;
    }
  }

  return false;
}


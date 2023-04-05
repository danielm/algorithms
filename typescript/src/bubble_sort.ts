// BigO: O(n^2)
export default function bubble_sort(arr: number[]): void {
  /*
   * So based on the coment below, these two loops are the ones that cause squared algorithm,
   * because we ar going over the length of the array, en every time we go over we go over again
   * some or most of the array everysinge time
   */
  for (let i = 0; i < arr.length; i++) {
    for (let j = 0; j < arr.length-1-i; j++) {
      /*
       * The runtime of accessin an alement of an array is O(1) -> Fixed computational cost
       * If we access it twice, like below, its just 2 Constant
       * So this is all constant operations, all here is constant
       * Nothing in here is based on input size.
       */
      if (arr[j] > arr[j+1]) {
        // Typescript wizzardy: https://www.typescriptlang.org/docs/handbook/variable-declarations.html#destructuring
        [arr[j], arr[j+1]] = [arr[j+1], arr[j]];
      }
    }
  }
}

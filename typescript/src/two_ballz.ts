// BigO: O(sqrt(N))

// We choose sqrt(N) because is the only way we can change from non-linear running
export function two_ballz(breaks: boolean[]): number {
  let jump = Math.floor(Math.sqrt(breaks.length));
  let i = jump;

  // Find first time is true (where is break)--> this doesnt garantees is the first, so we have
  // to jump back first (worst case)
  while (i < breaks.length) {
    if (breaks[i]) break;

    i +=jump;
  }

  // Now we jump back, and walk forward (linear search ackshhhualy)
  let end = i;
  i -= jump;

  while (i < end) {
    if (breaks[i]) {
      return i;
    }

    i += 1;
  }
  return -1;
}


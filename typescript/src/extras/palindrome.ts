const spanishSpecialMap: { [key: string]: string } = {
    'á': 'a',
    'é': 'e',
    'í': 'i',
    'ó': 'o',
    'ú': 'u',
    'ü': 'u',
    'ñ': 'n',
};

function clanupSpanishString(input: string): string {
    // Lowecase and transofrm spanish specific special chars
    const replacedChars = input.toLocaleLowerCase().replace(/./g, (char) => {
      return spanishSpecialMap[char] || char;
    });

    //remove all non alpha numeric characters, and lowercase the input
    return replacedChars.replace(/[^a-z0-9]/g, '');
  }

export function isPalindrome(input: string): boolean {
    input = clanupSpanishString(input);
    
    const len = input.length;

    // Special case... yep, dont we love them?
    if (len === 0) {
        return false;
    }

    const mid = Math.floor(len / 2);
    
    for (let i = 0; i < mid; i++) {
        if (input[i] !== input[len - 1 - i]) {
            return false;
        }
    }
    return true;
}
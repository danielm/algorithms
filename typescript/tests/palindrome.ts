import {expect, test} from '@jest/globals';
import { isPalindrome } from '../src/extras/palindrome';

test("palindromos españoles con caracteres especiales", function() {
    expect(isPalindrome('¿Acaso hubo búhos acá?')).toEqual(true);
    expect(isPalindrome('¿Son robos o sobornos?')).toEqual(true);
    expect(isPalindrome('uwu')).toEqual(true);
    expect(isPalindrome('uwwu')).toEqual(true);
    expect(isPalindrome('ukwu')).toEqual(false);
    expect(isPalindrome('')).toEqual(false);
});


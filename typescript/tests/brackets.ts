import {expect, test} from '@jest/globals';
import { okBrackets } from '../src/extras/brackets';

test("checking balanced brackets,parenthesis or.... yep, english is hard", function() {
    expect(okBrackets('{()}[]')).toEqual(true);
    expect(okBrackets('[(])')).toEqual(false);
    expect(okBrackets('{}')).toEqual(true);
    expect(okBrackets(']')).toEqual(false);
    expect(okBrackets('(')).toEqual(false);
});


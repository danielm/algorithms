import {expect, test} from '@jest/globals';
import { two_ballz } from '../src/two_ballz';

test("two crystal balls", function () {
    let idx = Math.floor(Math.random() * 10000);
    const data = new Array(10000).fill(false);

    for (let i = idx; i < 10000; ++i) {
        data[i] = true;
    }

    expect(two_ballz(data)).toEqual(idx);
    expect(two_ballz(new Array(821).fill(false))).toEqual(-1);
});

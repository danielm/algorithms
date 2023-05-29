import Stack from "../stack";

export function okBrackets(input: string): boolean {
    const list = new Stack<string>();

    for (let i = 0; i < input.length; i++) {
        if (list.length === 0) {
            list.push(input[i]);
            continue;
        }

        let curr = list.peek();
        
        if ((curr === '(' && input[i] === ')') ||
            (curr === '[' && input[i] === ']') ||
            (curr === '{' && input[i] === '}')
        ) {
            // Found a pair
            list.pop();
            continue;
        }

        // Bubble buble buble push!
        list.push(input[i]);
    }

    // If the stack is empty, then we got all matching!
    return list.length === 0;
}
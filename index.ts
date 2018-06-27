import {add, say_hello} from './some-lib/src/lib.rs';

console.log('Initializing...');

console.log('Adding:' + add(5, 5));

const $content = document.getElementById('content');

// Doesn't work because WASM can only return numbers.
console.log(say_hello());

$content.innerHTML = "Result: "+add(40, 2)+"; Phrase: " +say_hello();

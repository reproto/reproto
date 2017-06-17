import {Entry} from './test.js';
import * as readline from 'readline';

var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

rl.on('line', function(line) {
  let data = JSON.parse(line);
  console.log(Entry.decode(data));
})

const inputArr = require('./part1');

let currentFloor = 0;

for (let i = 0; i < inputArr.length; i++) {
  if (inputArr[i] === '(') {
    currentFloor++;
  } else if (inputArr[i] === ')') {
    currentFloor--;
  }
  if (currentFloor === -1) {
    console.log(i + 1);
    break;
  }
}

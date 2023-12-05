import fs from 'fs';
import { log } from 'console';

const lines = fs.readFileSync('./day1.txt', { encoding: 'utf-8' }).split('\n');

function isDigit(char) {
  return /^\d$/.test(char);
}

function part1() {
  let sum = 0;

  for (const line of lines) {
    let firstNum, lastNum;
    for (const c of line) {
      if (isDigit(c)) {
        if (!firstNum) {
          firstNum = c;
        }
        lastNum = c;
      }
    }

    const twoDigit = `${firstNum}${lastNum}`;
    sum += Number(twoDigit);
  }

  log(sum);
}

function part2() {
  let sum = 0;

  for (const line of lines) {
    let digits = [];

    for (let i = 0; i < line.length; i++) {
      const c = line[i];

      if (isDigit(c)) {
        digits.push(c);
      }

      const textDigits = [
        'one',
        'two',
        'three',
        'four',
        'five',
        'six',
        'seven',
        'eight',
        'nine',
      ];
      const lineSubstring = line.substring(i);
      for (let d = 0; d < textDigits.length; d++) {
        const textDigit = textDigits[d];
        if (lineSubstring.startsWith(textDigit)) {
          digits.push(d + 1);
        }
      }
    }

    const lastIndex = digits.length - 1;
    const twoDigits = `${digits[0]}${digits[lastIndex]}`;

    sum += Number(twoDigits);
  }

  log(sum);
}

part1();
part2();

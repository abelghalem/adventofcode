import chalk from 'chalk';
import { readData } from '../../shared.ts';

const numbers = [
  '1',
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  '0',
  'one',
  'two',
  'three',
  'four',
  'five',
  'six',
  'seven',
  'eight',
  'nine',
  'zero',
];

const numbersMap = {
  '1': 1,
  '2': 2,
  '3': 3,
  '4': 4,
  '5': 5,
  '6': 6,
  '7': 7,
  '8': 8,
  '9': 9,
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

function getFirstNumber(value: string) {
  for (let i = 0; i < value.length; i++) {
    for (const number of numbers) {
      if (value.startsWith(number, i)) {
        return numbersMap[number];
      }
    }
  }

  throw new Error('No number');
}

function getLastNumber(value: string) {
  for (let i = 0; i < value.length; i++) {
    for (const number of numbers) {
      if (value.endsWith(number, value.length - i)) {
        return numbersMap[number];
      }
    }
  }

  throw new Error('No number');
}

export async function day1a(dataPath?: string) {
  const data = await readData(dataPath);

  let total = 0;

  for (const value of data) {
    total += parseInt(`${getFirstNumber(value)}${getLastNumber(value)}`);
  }

  return total;
}

const answer = await day1a();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

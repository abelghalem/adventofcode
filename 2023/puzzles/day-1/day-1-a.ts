import chalk from 'chalk';
import { readData } from '../../shared.ts';

function isNumber(value: string) {
  return ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].includes(value);
}

function getFirstNumber(value: string) {
  for (let i = 0; i < value.length; i++) {
    if (isNumber(value.at(i))) {
      return value.at(i);
    }
  }

  throw new Error('No number');
}

function getLastNumber(value: string) {
  for (let i = 1; i <= value.length; i++) {
    if (isNumber(value.at(-i))) {
      return value.at(-i);
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

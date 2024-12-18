import chalk from 'chalk';
import { readData } from '../../shared.ts';

type Point = {
  line: number;
  column: number;
};

type NumberInstance = Point & {
  value: number;
  length: number;
  slug: string;
};

type SignInstance = Point;

function checkLine(
  numbersMap: Array<NumberInstance>,
  line: number,
  column: number
) {
  const numbers = numbersMap.filter((n) => n.line === line);
  const results: Array<NumberInstance> = [];

  if (!numbers.length) {
    return results;
  }

  for (const n of numbers) {
    if (column >= n.column - 1 && column <= n.column + n.length) {
      results.push(n);
    }
  }

  return results;
}

function findCloseNumbers(
  numbersMap: Array<NumberInstance>,
  line: number,
  column: number
) {
  const results: Array<NumberInstance> = [];

  if (line > 0) {
    results.push(...checkLine(numbersMap, line - 1, column));
  }

  results.push(...checkLine(numbersMap, line, column));
  results.push(...checkLine(numbersMap, line + 1, column));

  if (results.length !== 2) {
    return [];
  }

  return results;
}

export async function day3b(dataPath?: string) {
  const data = await readData(dataPath);

  const numbersMap: Array<NumberInstance> = [];
  const signsMap: Array<SignInstance> = [];

  for (const lineIndex in data) {
    const line = data[lineIndex];
    const numberGroups = [...line.matchAll(/([0-9]+)/g)];

    for (const numberGroup of numberGroups) {
      const value = parseInt(numberGroup[0]);
      const line = parseInt(lineIndex);
      const column = numberGroup.index;
      const length = numberGroup[0].length;
      numbersMap.push({
        value,
        line,
        column,
        length,
        slug: `${value}-${line}-${column}-${length}`,
      });
    }

    const signGroups = [...line.matchAll(/([\*]+)/g)];

    for (const signGroup of signGroups) {
      signsMap.push({
        line: parseInt(lineIndex),
        column: signGroup.index,
      });
    }
  }

  const gears: Array<NumberInstance[]> = [];

  for (const { line, column } of signsMap) {
    gears.push(findCloseNumbers(numbersMap, line, column));
  }

  let result = 0;

  for (const parts of gears) {
    if (parts.length) {
      let total = parts.reduce((total, n) => total * n.value, 1);
      result += total;
    }
  }

  return result;
}

const answer = await day3b();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

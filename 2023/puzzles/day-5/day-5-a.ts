import { readData } from '../../shared.ts';
import chalk from 'chalk';

export async function day5a(dataPath?: string) {
  const data = await readData(dataPath);

  for (const line of data) {
    console.log(line);
  }

  return 0;
}

const answer = await day5a();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

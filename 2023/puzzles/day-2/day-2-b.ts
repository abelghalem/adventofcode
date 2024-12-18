import chalk from 'chalk';
import { readData } from '../../shared.ts';

const redCubes = 12;
const greenCubes = 13;
const blueCubes = 14;

export async function day2b(dataPath?: string) {
  const data = await readData(dataPath);

  let total = 0;

  for (const game of data) {
    const minCubes = {
      red: -1,
      green: -1,
      blue: -1,
    };

    const gameId = parseInt(game.split(':')[0].split(' ')[1]);
    const parts = game.slice(game.indexOf(': ') + 2).split(';');

    for (const part of parts) {
      const cubePulls = part.split(', ');

      for (const cubePull of cubePulls) {
        const cubePullParts = cubePull.trim().split(' ');
        const number = parseInt(cubePullParts[0]);
        const color = cubePullParts[1];

        if (number > minCubes[color]) {
          minCubes[color] = number;
        }
      }
    }

    console.log(minCubes.red * minCubes.green * minCubes.blue);
    console.log('################');

    total += minCubes.red * minCubes.green * minCubes.blue;
  }

  return total;
}

const answer = await day2b();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

import chalk from 'chalk';
import { readData } from '../../shared.ts';

const redCubes = 12;
const greenCubes = 13;
const blueCubes = 14;

export async function day2a(dataPath?: string) {
  const data = await readData(dataPath);

  let total = 0;

  for (const game of data) {
    const maxCubes = {
      red: 0,
      green: 0,
      blue: 0,
    };

    const gameId = parseInt(game.split(':')[0].split(' ')[1]);
    const parts = game.slice(game.indexOf(': ') + 2).split(';');
    for (const part of parts) {
      const cubePulls = part.split(', ');

      for (const cubePull of cubePulls) {
        const cubePullParts = cubePull.trim().split(' ');
        const number = parseInt(cubePullParts[0]);
        const color = cubePullParts[1];

        if (maxCubes[color] === -1 || maxCubes[color] < number) {
          maxCubes[color] = number;
        }
      }
    }

    if (
      maxCubes.red <= redCubes &&
      maxCubes.green <= greenCubes &&
      maxCubes.blue <= blueCubes
    ) {
      total += gameId;
    }
  }

  return total;
}

const answer = await day2a();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

import chalk from 'chalk';
import { readData } from '../../shared.ts';

type Card = {
  id: number;
  winningNumbers: number[];
  playedNumbers: number[];
};

export async function day4a(dataPath?: string) {
  const data = await readData(dataPath);

  const cards: Array<Card> = [];
  for (const line of data) {
    const [cardNumber, numbers] = line.split(': ');
    const [winningNumbers, playedNumbers] = numbers.split(' | ');
    const id = parseInt(cardNumber.split(' ')[1]);

    cards.push({
      id,
      winningNumbers: winningNumbers
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
      playedNumbers: playedNumbers
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
    });
  }

  let total = 0;

  for (const card of cards) {
    const validNumbers = card.playedNumbers.filter((n) =>
      card.winningNumbers.includes(n)
    );

    if (validNumbers.length) {
      total += Math.pow(2, validNumbers.length - 1);
    }
  }

  return total;
}

const answer = await day4a();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

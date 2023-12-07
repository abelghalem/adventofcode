import chalk from 'chalk';
import { readData } from '../../shared.ts';

type Card = {
  id: number;
  winningNumbers: number[];
  playedNumbers: number[];
  count: number;
};

export async function day4b(dataPath?: string) {
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
      count: 1,
    });
  }

  let total = 0;

  for (const cardIndex in cards) {
    const card = cards[cardIndex];

    const validNumbers = card.playedNumbers.filter((n) =>
      card.winningNumbers.includes(n)
    );

    if (validNumbers.length) {
      for (let j = 0; j < card.count; j++) {
        for (let i = 1; i <= validNumbers.length; i++) {
          if (parseInt(cardIndex) === 0) {
            console.log(`adding count to card ${parseInt(cardIndex) + i + 1}`);
          }
          cards[parseInt(cardIndex) + i].count++;
        }
      }
    }
  }

  return cards.reduce((total, card) => total + card.count, 0);
}

const answer = await day4b();
console.log(chalk.bgGreen('Your Answer:'), chalk.green(answer));

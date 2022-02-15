# Tower of Hanoi - the CLI game

You can play this game online on [replit](https://replit.com/@fswerneck/hanoi-tower-rust-v2).

## Objective
Move all the discs from tower **1** to tower **3**.

## Game dynamics
There are three towers and five discs. Each disc has a size, ranging from 1 to
\5. A smaller disc can stand on a larger disc, but not the other way around.
That means you can place, e.g., a disc of size 1 on top of a disc of size 3,
but not a disc of size 5 on top of a disc of size 2. You have to move all five
discs on tower 1 to tower 3, using tower 2 as an intermediate to move them.

## How to play
You'll be prompted to select a tower to take a disc from, or drop a disc onto.
As you press any of **1**, **2**, or **3**, your virtual hand will grab a disc
from, or drop one on the selected tower. The game ends when all discs are on the
third tower.

In case you want to quit, **q** is the input of choice.

## Background
TODO: tell the whys behind this repo.

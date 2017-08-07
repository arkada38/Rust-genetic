# Rust-genetic
Genetic algorithm for matching strings

## Steps
1. The user enters a string (*expected*).
2. Algorithm generates random string (*actual*) with same length as *expected* string.
3. Algorithm generates population with 100 members. Each member have an *actual* string with one mutable character.
4. Members are sorted according to the best match with the *expected* string.
5. The population reproduces while the string of the best member does not match the *expected*.
6. The algorithm outputs the result.

## Reproducing of the population
1. Each best of the ten chooses a partner from the best half of the population ten times.
2. Children's string is a randomly mixed strings of parents.

## Facts
1. The top ten have at least ten children each.
2. The next population consists only of children.
3. The size of the population is always 100.
4. The expected string may contain english and russian characters.

## Example
![Screen](img/genetic.gif)
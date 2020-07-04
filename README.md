# msolve
[![Travis CI](https://api.travis-ci.org/mkb2091/msolve.svg?branch=master)](https://travis-ci.org/github/mkb2091/msolve/builds)

A WIP sudoku solving library

# Goals

Improve performance, potentially via using SIMD, and or GPU

Add sudoku difficulty grading, potentially by counting the number of iterations needed to solve

Add sudoku generation from a seed sudoku 

# Usage 

To get the first solution to inputted sudoku list: msolve solve_one < sudokufile.txt > output.txt

To get the solutions of all puzzles with a unique solution:  msolve solve_unique < sudokufile.txt > output.txt

To get the first N solutions:  msolve solve_n N < sudokufile.txt > output.txt

To get information about the sudoku file: msolve info < sudokufile.txt

Example output for info:
  0 Solutions: 486451, 1 Solution: 763, 2+ Solutions: 12786

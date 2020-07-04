# msolve
[![Travis CI](https://api.travis-ci.org/mkb2091/msolve.svg?branch=master)](https://travis-ci.org/github/mkb2091/msolve/builds)

A WIP sudoku solving library

# Goals

Improve performance, potentially via using SIMD, and or GPU

Add sudoku difficulty grading, potentially by counting the number of iterations needed to solve

Add sudoku generation from a seed sudoku 

# Usage 

To get the first solution to each sudoku in input: msolve solve_one < sudokufile.txt > output.txt

To get the solution for each uniquely solvable sudoku in input:  msolve solve_unique < sudokufile.txt > output.txt

To get the first N solutions to each sudoku in input: msolve solve_n N < sudokufile.txt > output.txt

To get information about the sudoku file: msolve info < sudokufile.txt

Example output for info:
  0 Solutions: 486451, 1 Solution: 763, 2+ Solutions: 12786

To list all puzzles with a single unique solution: msolve find_with_single_solution < sudokufile.txt > output.txt

To list all puzzles with at least one unique solution: msolve find_with_solution < sudokufile.txt > output.txt

To get the number of solutions of each puzzles, up to a maximum of N: msolve count_solutions N < sudokufile.txt > output.txt
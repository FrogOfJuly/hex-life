# Hexagonal game of life on a sphere

It is a pet project for my own amusement. I did it because I like hexagonal grids on spheres and cellular automata. 

Inspiration is taken from [here](https://arunarjunakani.github.io/HexagonalGameOfLife). Rules: `3,5/2`, are taken from [here](https://content.wolfram.com/uploads/sites/13/2018/02/15-3-4.pdf)

# Patterns

In more human-readable form rules are as follows:

* Cell appears if there are 2 neighbors
* Cell survives if there are 3 or 5 neighbors

There are more stable patterns here than in classical Conway's Game Of Life, but this variation converges too quickly creating a less rich environment. 

The most notable pattern is a glider. You can try all of them [here](https://frogofjuly.github.io/hex-life/).

If you know some cool patterns and want me to add them - open an issue or a pull request. The transition function for patterns is from [here](https://github.com/HydroniumLabs/h3o/issues/15).
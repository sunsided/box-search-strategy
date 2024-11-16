# 📦🔍 Box Searching simulation

> Eight boxes are filled with two coins, where all boxes have equal
probability to have a coin. Alice searches the boxes left to right,
then row by row, whereas Bob searches the boxes top to bottom,
column by column. The first person to find a coin wins.
Are Alice and Bob equally likely to win?
> 
> ```text
> _________________
> |   |   | 🪙 |   |
> -----------------
> |   | 🪙 |   |   |
> -----------------
> ```

One could argue that if at least one coin is located in the top row, Alice
has an edge over Bob. If all coins are in the second row, Bob has an edge over
alice. Since the distribution of coins is uniform, they might have equal chance.

This is incorrect, which might feel paradoxical at first glance due to our
intuitive assumptions about symmetry and fairness. Here's a breakdown:

1. **Symmetry Intuition:**
  At first glance, Alice and Bob seem to have symmetric roles. Both are searching for coins in the same grid, with the same probabilities assigned to each box containing a coin.
  Symmetry often leads us to expect equal chances of success, but the search order introduces an asymmetry in their strategies.
2. **Traversal Bias:**
  Alice searches left-to-right, row-by-row, while Bob searches top-to-bottom, column-by-column. The grid layout means their traversal paths overlap in non-intuitive ways. This can result in uneven probabilities of finding the coin first.
3. **Dependence of Outcomes:**
  If the coins are distributed probabilistically, the interaction between Alice's and Bob's traversal paths can create non-uniform probabilities for who finds a coin first, leading to unexpected results.

The outcome is not actually paradoxical as it is fully determined by
* The distribution of coins in the grid.
* The deterministic traversal strategies of Alice and Bob.

## Simulation output

Here's the example output of the simulation for Alice (row-wise search order),
Bob (column-wise search order) and Charlie (random order with uniform distribution):

```text
Outcome after for 2 rows, 8 columns, 2 coins (100000 trials)
  row-wise wins:     46886 (37.76%)
  column-wise wins:  33631 (27.09%)
  random-order wins: 43641 (35.15%)
Outcome after for 8 rows, 2 columns, 2 coins (100000 trials)
  row-wise wins:     33508 (26.97%)
  column-wise wins:  46577 (37.49%)
  random-order wins: 44157 (35.54%)
Outcome after for 8 rows, 8 columns, 2 coins (100000 trials)
  row-wise wins:     36488 (33.29%)
  column-wise wins:  36507 (33.31%)
  random-order wins: 36599 (33.40%)
Outcome after for 8 rows, 8 columns, 4 coins (100000 trials)
  row-wise wins:     38292 (33.50%)
  column-wise wins:  37917 (33.17%)
  random-order wins: 38088 (33.32%)
```

From this we find that searching along the longest dimension has an edge
over searching along the shortest dimension. We also find that a randomized search
appears to be on par with the optimal directional strategy for the according
grid layout. This is because the coin distribution is randomized as well.
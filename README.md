# 📦🔍 Box Searching simulation

> Eight boxes are filled with two coins, where all boxes have equal
probability to have a coin. Alice searches the boxes left to right,
then row by row, whereas Bob searches the boxes top to bottom,
column by column. The first person to find a coin wins.
Are Alice and Bob equally likely to win?
> 
> ```text
> _________________
> |   |   | C |   |
> -----------------
> |   | C |   |   |
> -----------------
> ```

This problem is not inherently paradoxical, but it might feel paradoxical at first glance due to our intuitive assumptions about symmetry and fairness. Here's a breakdown:

**Why it might seem paradoxical:**

1. **Symmetry Intuition:**
  At first glance, Alice and Bob seem to have symmetric roles. Both are searching for coins in the same grid, with the same probabilities assigned to each box containing a coin.
  Symmetry often leads us to expect equal chances of success, but the search order introduces an asymmetry in their strategies.
2. **Traversal Bias:**
  Alice searches left-to-right, row-by-row, while Bob searches top-to-bottom, column-by-column. The grid layout means their traversal paths overlap in non-intuitive ways. This can result in uneven probabilities of finding the coin first.
3. **Dependence of Outcomes:**
  If the coins are distributed probabilistically, the interaction between Alice's and Bob's traversal paths can create non-uniform probabilities for who finds a coin first, leading to unexpected results.

**Why it's ultimately straightforward:**

The outcome is not actually paradoxical as it is fully determined by
* The distribution of coins in the grid.
* The deterministic traversal strategies of Alice and Bob.

## Simulation output

Here's the example output of the simulation:

```text
Outcome after for 2 rows, 8 columns, 2 coins (100000 trials)
  row-wise wins:    50984 (60.43%)
  column-wise wins: 33383 (39.57%)
  ties:             15633 (18.53%)
Outcome after for 8 rows, 2 columns, 2 coins (100000 trials)
  row-wise wins:    33516 (39.77%)
  column-wise wins: 50759 (60.23%)
  ties:             15725 (18.66%)
Outcome after for 8 rows, 8 columns, 2 coins (100000 trials)
  row-wise wins:    44579 (50.16%)
  column-wise wins: 44288 (49.84%)
  ties:             11133 (12.53%)
Outcome after for 8 rows, 8 columns, 4 coins (100000 trials)
  row-wise wins:    43195 (49.82%)
  column-wise wins: 43500 (50.18%)
  ties:             13305 (15.35%)
```
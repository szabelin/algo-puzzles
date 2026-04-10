# Problem 2: Egg Drop

*Minimax DP · Worst-Case Optimization*

[← Back to main README](../../README.md) · [▶ Try it in the Interactive Playground](https://szabelin.github.io/algo-puzzles/#eggdrop)

## Setup

You have **k eggs** and a building with **n floors**.
There exists a critical floor **f** (0 ≤ f ≤ n) such that:
- Dropping an egg from floor > f → egg breaks
- Dropping an egg from floor ≤ f → egg survives

**What is the minimum number of trials to *guarantee* finding f?**

## The Recurrence

```
T(k, n) = 1 + min over x ∈ [1..n] of:
             max( T(k−1, x−1),    ← egg breaks, search below
                  T(k,   n−x) )   ← egg survives, search above

Base cases:
  T(k, 0) = 0     — no floors to check
  T(k, 1) = 1     — drop once from floor 1
  T(1, n) = n     — one egg, must check every floor bottom-up
```

## Why Recursion DOES Work (Unlike Gambler's Ruin)

The dependency graph is a **DAG** — every recursive call strictly reduces
toward a base case:

```
T(k, n) → T(k−1, x−1)    k decreases
T(k, n) → T(k,   n−x)    n decreases (since x ≥ 1)
```

No state ever depends on itself, directly or indirectly. So memoized
recursion works perfectly. This repo implements **three solutions**:

| Method | Time | Space | Approach |
|--------|------|-------|----------|
| Recursive + Memo | O(kn²) | O(kn) | Top-down, memoized |
| Iterative Bottom-Up | O(kn²) | O(kn) | Classic DP table |
| Binary Search Optimized | O(kn log n) | O(kn) | Exploit monotonicity of the split |

All three are proven to give identical results via tests.

## Classic Results

| Eggs | Floors | Min Trials |
|------|--------|------------|
| 1    | 100    | 100        |
| 2    | 10     | 4          |
| 2    | 100    | 14         |
| 3    | 25     | 5          |
| 3    | 100    | 9          |
| 10   | 1000   | 10         |

## The Binary Search Insight

As `x` increases, `T(k−1, x−1)` increases monotonically (more floors
below to check with fewer eggs) and `T(k, n−x)` decreases monotonically
(fewer floors above). The optimal split `x` is where these two curves
cross — findable via binary search, dropping the inner loop from O(n)
to O(log n).

## Complexity

- **Naive:** O(k × n²) — try all floors at each state
- **Optimized:** O(k × n log n) — binary search on the split point
- **Space:** O(k × n)

## Source

- [`src/problems/egg_drop.rs`](../../src/problems/egg_drop.rs)

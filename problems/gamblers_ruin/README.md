# Problem 1: Gambler's Ruin

*Bellman Equation · Value Iteration · Markov Decision Process*

[← Back to main README](../../README.md) · [▶ Try it in the Interactive Playground](https://szabelin.github.io/algo-puzzles/#gambler)

## Setup

You have **$x** and want to reach a target **$T**.
Each round, you bet **$b** (where `1 ≤ b ≤ min(x, T−x)`).
You win the bet with probability **p**, lose with probability **1−p**.

**What is your optimal strategy, and what is your probability of reaching $T?**

## The Recurrence (Bellman Equation)

```
r(x) = max over b ∈ [1..min(x, T−x)] of:
          p · r(x + b) + (1 − p) · r(x − b)

Base cases:
  r(0) = 0    — broke, game over
  r(T) = 1    — reached target, you win
```

## Why Recursion Does NOT Work

This is the key insight — and the reason this problem is interesting.

In most DP problems, the dependency graph is a **DAG**: subproblems always
shrink toward base cases. Gambler's Ruin is different.

Consider `r(500)` with `T = 1000`:

```
r(500) depends on r(501)   ← bet 1, win
r(501) depends on r(502)   ← bet 1, win
  ...
r(998) depends on r(999)   ← bet 1, win
r(999) depends on r(998)   ← bet 1, lose  ← CYCLE!
```

States form **cycles**: `r(A)` needs `r(B)` which needs `r(A)`.

| Approach | Works? | Why |
|----------|--------|-----|
| Pure recursion | ✗ | Infinite loop / stack overflow |
| Memoized recursion | ✗ | Returns wrong values for uncached cyclic states |
| Topological sort | ✗ | Impossible — the graph has cycles |
| **Value iteration** | **✓** | Sweep repeatedly until convergence |

The **only** correct method is **iterative value iteration**: initialize
a guess, sweep through all states, update each one using the current
estimates of its neighbors, and repeat until convergence.

The Rust source includes a `solve_recursive_BROKEN` function that
demonstrates exactly how and why recursion fails — run `cargo test`
to see it break.

## Key Result: Bold Play is Optimal

When `p < 0.5` (the house has an edge), the optimal strategy is
**bold play** — always bet the maximum possible amount. Intuitively:
you're in a losing game, so minimize the number of rounds you play.

## Complexity

- **Time:** O(iterations × T × max_bet) ≈ O(I × T²)
- **Space:** O(T)

## Source

- [`src/problems/gamblers_ruin.rs`](../../src/problems/gamblers_ruin.rs)

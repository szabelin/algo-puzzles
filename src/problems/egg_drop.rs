/// # Egg Drop Problem — Minimax DP
///
/// ## Problem
/// You have `k` eggs and a building with `n` floors.
/// Find the minimum number of trials to guarantee finding the critical floor.
///
/// ## The Recurrence
///
/// ```text
///   T(k, n) = 1 + min over x in [1..n] of:
///               max( T(k-1, x-1),   // egg breaks → search below
///                    T(k,   n-x) )   // egg survives → search above
///
///   Base cases:
///     T(k, 0) = 0, T(k, 1) = 1, T(1, n) = n
/// ```
///
/// ## Why Recursion DOES Work Here (Unlike Gambler's Ruin)
///
/// The dependency graph is a DAG:
///   - T(k-1, x-1): fewer eggs AND fewer floors
///   - T(k, n-x):   same eggs but fewer floors (n-x < n since x ≥ 1)
///
/// No cycles. Memoized recursion works perfectly.

use std::collections::HashMap;

/// Result from solving the Egg Drop problem.
#[derive(Debug, Clone)]
pub struct EggDropResult {
    /// table[k][n] = minimum trials with k eggs and n floors
    pub table: Vec<Vec<usize>>,
    /// The answer for the original (eggs, floors)
    pub min_trials: usize,
}

// ═══════════════════════════════════════════════════════════════
//  SOLUTION 1: Recursive + Memoization (Top-Down)
//
//  Works because the dependency graph is a DAG.
//  Every call reduces either k or n toward a base case.
// ═══════════════════════════════════════════════════════════════

pub fn solve_recursive(eggs: usize, floors: usize) -> EggDropResult {
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    fn dp(
        k: usize,
        n: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        if k == 1 { return n; }

        if let Some(&cached) = memo.get(&(k, n)) {
            return cached;
        }

        let mut best = usize::MAX;
        for x in 1..=n {
            let breaks = dp(k - 1, x - 1, memo);
            let survives = dp(k, n - x, memo);
            let worst_case = 1 + breaks.max(survives);
            best = best.min(worst_case);
        }

        memo.insert((k, n), best);
        best
    }

    let min_trials = dp(eggs, floors, &mut memo);

    // Build full table for visualization
    let mut table = vec![vec![0usize; floors + 1]; eggs + 1];
    for k in 1..=eggs {
        for n in 0..=floors {
            table[k][n] = dp(k, n, &mut memo);
        }
    }

    EggDropResult { table, min_trials }
}

// ═══════════════════════════════════════════════════════════════
//  SOLUTION 2: Iterative Bottom-Up DP
//
//  Same logic, no recursion overhead, no stack limits.
// ═══════════════════════════════════════════════════════════════

pub fn solve_iterative(eggs: usize, floors: usize) -> EggDropResult {
    let mut table = vec![vec![0usize; floors + 1]; eggs + 1];

    // Base: 1 egg → must try every floor linearly
    for n in 0..=floors {
        if eggs >= 1 {
            table[1][n] = n;
        }
    }

    // Base: 1 floor → always 1 trial
    for k in 1..=eggs {
        if floors >= 1 { table[k][1] = 1; }
    }

    for k in 2..=eggs {
        for n in 2..=floors {
            table[k][n] = usize::MAX;
            for x in 1..=n {
                let worst_case = 1 + table[k - 1][x - 1].max(table[k][n - x]);
                table[k][n] = table[k][n].min(worst_case);
            }
        }
    }

    let min_trials = table[eggs][floors];
    EggDropResult { table, min_trials }
}

// ═══════════════════════════════════════════════════════════════
//  SOLUTION 3: Binary Search Optimized — O(k * n * log n)
//
//  T(k-1, x-1) increases monotonically in x.
//  T(k, n-x) decreases monotonically in x.
//  Optimal x is at the crossover → binary search.
// ═══════════════════════════════════════════════════════════════

pub fn solve_optimized(eggs: usize, floors: usize) -> EggDropResult {
    let mut table = vec![vec![0usize; floors + 1]; eggs + 1];

    for n in 0..=floors {
        if eggs >= 1 { table[1][n] = n; }
    }
    for k in 1..=eggs {
        if floors >= 1 { table[k][1] = 1; }
    }

    for k in 2..=eggs {
        for n in 2..=floors {
            let mut lo = 1;
            let mut hi = n;
            let mut best = usize::MAX;

            // Binary search for the optimal floor x to drop from.
            // Key insight: T(k-1, x-1) increases monotonically in x (more floors
            // below → more work if egg breaks), while T(k, n-x) decreases
            // monotonically in x (fewer floors above → less work if egg survives).
            // The optimal x minimizes max(breaks, survives) — at the crossover point.
            while lo <= hi {
                let mid = (lo + hi) / 2;
                let breaks = table[k - 1][mid - 1];
                let survives = table[k][n - mid];
                let worst = 1 + breaks.max(survives);
                best = best.min(worst);

                // Move toward the crossover: if breaks < survives, try higher floor;
                // if breaks > survives, try lower floor; if equal, we're at optimum.
                if breaks < survives {
                    lo = mid + 1;
                } else if breaks > survives {
                    if mid == 0 { break; }
                    hi = mid - 1;
                } else {
                    break;
                }
            }

            table[k][n] = best;
        }
    }

    let min_trials = table[eggs][floors];
    EggDropResult { table, min_trials }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cases() {
        let r = solve_iterative(3, 10);
        assert_eq!(r.table[1][0], 0);
        assert_eq!(r.table[1][1], 1);
        assert_eq!(r.table[2][0], 0);
    }

    #[test]
    fn test_one_egg() {
        let r = solve_iterative(1, 50);
        for n in 0..=50 {
            assert_eq!(r.table[1][n], n);
        }
    }

    #[test]
    fn test_known_values() {
        assert_eq!(solve_iterative(2, 10).min_trials, 4);
        assert_eq!(solve_iterative(2, 100).min_trials, 14);
        assert_eq!(solve_iterative(3, 25).min_trials, 5);
    }

    #[test]
    fn test_recursive_matches_iterative() {
        for eggs in 1..=4 {
            for floors in 0..=30 {
                let rec = solve_recursive(eggs, floors);
                let iter = solve_iterative(eggs, floors);
                assert_eq!(
                    rec.min_trials, iter.min_trials,
                    "Mismatch at eggs={}, floors={}",
                    eggs, floors
                );
            }
        }
    }

    #[test]
    fn test_optimized_matches_iterative() {
        for eggs in 1..=4 {
            for floors in 0..=50 {
                let opt = solve_optimized(eggs, floors);
                let iter = solve_iterative(eggs, floors);
                assert_eq!(
                    opt.min_trials, iter.min_trials,
                    "Mismatch at eggs={}, floors={}",
                    eggs, floors
                );
            }
        }
    }

    #[test]
    fn test_more_eggs_never_hurts() {
        let floors = 30;
        let mut prev = usize::MAX;
        for k in 1..=6 {
            let r = solve_iterative(k, floors);
            assert!(r.min_trials <= prev);
            prev = r.min_trials;
        }
    }
}

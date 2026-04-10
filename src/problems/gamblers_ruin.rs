/// # Gambler's Ruin — Bellman Equation / Value Iteration
///
/// ## Problem
/// You have $x and want to reach a target $T.
/// Each round you bet $b (where 1 ≤ b ≤ min(x, T-x)).
/// You win the bet with probability p and lose with probability (1-p).
///
/// ## The Recurrence (Bellman Equation)
///
/// ```text
///   r(x) = max over b in [1..min(x, T-x)] of:
///            p * r(x + b) + (1 - p) * r(x - b)
///
///   Base cases:
///     r(0) = 0.0   — you're broke, game over
///     r(T) = 1.0   — you reached the target, you win
/// ```
///
/// ## Why Recursion Does NOT Work Here
///
/// In most DP problems (knapsack, edit distance, egg drop), the dependency
/// graph is a DAG — subproblems always reduce toward base cases.
///
/// Gambler's Ruin is different. Consider r(500) with T=1000:
///   - r(500) depends on r(501)  (bet 1, win)
///   - r(501) depends on r(502)  (bet 1, win)
///   - ...
///   - r(999) depends on r(998)  (bet 1, lose)  ← CYCLE
///
/// States form cycles: r(A) needs r(B) which needs r(A).
///   ✗ Pure recursion → infinite loop / stack overflow
///   ✗ Memoized recursion → returns wrong values for uncached cyclic states
///   ✗ Topological sort → impossible, the graph has cycles
///   ✓ Value iteration → sweep repeatedly until convergence

/// Result of solving the Gambler's Ruin problem.
#[derive(Debug, Clone)]
pub struct GamblersRuinResult {
    /// r[x] = probability of reaching target from state x
    pub probabilities: Vec<f64>,
    /// best_bet[x] = optimal bet size when you have $x
    pub best_bets: Vec<usize>,
    /// Number of iterations until convergence
    pub iterations: usize,
}

/// Solve Gambler's Ruin using iterative value iteration.
///
/// This is the ONLY correct method. See module docs for why recursion fails.
pub fn solve_iterative(
    target: usize,
    p_win: f64,
    tolerance: f64,
    max_iters: usize,
) -> GamblersRuinResult {
    let p_lose = 1.0 - p_win;

    // Initialize with linear interpolation as a starting guess
    let mut r: Vec<f64> = (0..=target)
        .map(|x| x as f64 / target as f64)
        .collect();

    // Base cases (never change)
    r[0] = 0.0;
    r[target] = 1.0;

    let mut best_bets = vec![0usize; target + 1];
    let mut iterations = 0;

    for iter in 0..max_iters {
        let mut max_delta: f64 = 0.0;

        for x in 1..target {
            let max_bet = x.min(target - x);
            let mut best_val: f64 = 0.0;
            let mut best_b: usize = 1;

            for b in 1..=max_bet {
                let val = p_win * r[x + b] + p_lose * r[x - b];
                if val > best_val {
                    best_val = val;
                    best_b = b;
                }
            }

            max_delta = max_delta.max((r[x] - best_val).abs());
            r[x] = best_val;
            best_bets[x] = best_b;
        }

        iterations = iter + 1;
        if max_delta < tolerance {
            break;
        }
    }

    GamblersRuinResult {
        probabilities: r,
        best_bets,
        iterations,
    }
}

/// Demonstrates WHY recursion does not work here.
///
/// When it hits a cycle, there is no valid value to return — so the
/// entire computation breaks. DO NOT USE.
/// Exists only as a teaching example. Run `cargo test` to see it break.
#[allow(dead_code, non_snake_case)]
pub fn solve_recursive_BROKEN(target: usize, p_win: f64) -> Vec<f64> {
    use std::collections::HashMap;

    fn helper(
        x: usize,
        target: usize,
        p_win: f64,
        memo: &mut HashMap<usize, f64>,
        in_progress: &mut Vec<bool>,
    ) -> f64 {
        if x == 0 {
            return 0.0;
        }
        if x >= target {
            return 1.0;
        }
        if let Some(&cached) = memo.get(&x) {
            return cached;
        }

        // CYCLE DETECTED — returning 0.0 is WRONG but what else can we do?
        // This is the fundamental problem with recursion on cyclic graphs.
        if in_progress[x] {
            return 0.0;
        }

        in_progress[x] = true;

        let max_bet = x.min(target - x);
        let mut best_val: f64 = 0.0;

        for b in 1..=max_bet {
            let val = p_win * helper(x + b, target, p_win, memo, in_progress)
                + (1.0 - p_win) * helper(x - b, target, p_win, memo, in_progress);
            if val > best_val {
                best_val = val;
            }
        }

        in_progress[x] = false;
        memo.insert(x, best_val);
        best_val
    }

    let mut memo = HashMap::new();
    let mut in_progress = vec![false; target + 1];
    let mut results = vec![0.0; target + 1];
    for x in 0..=target {
        results[x] = helper(x, target, p_win, &mut memo, &mut in_progress);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cases() {
        let result = solve_iterative(100, 0.49, 1e-12, 1000);
        assert!((result.probabilities[0] - 0.0).abs() < 1e-10);
        assert!((result.probabilities[100] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_fair_game() {
        // With p=0.5 (fair coin), r(x) = x/T exactly
        let result = solve_iterative(100, 0.5, 1e-12, 1000);
        for x in 0..=100 {
            let expected = x as f64 / 100.0;
            assert!(
                (result.probabilities[x] - expected).abs() < 1e-6,
                "r({}) = {}, expected {}",
                x, result.probabilities[x], expected
            );
        }
    }

    #[test]
    fn test_unfavorable_odds_monotonic() {
        let result = solve_iterative(100, 0.49, 1e-12, 1000);
        for x in 1..=100 {
            assert!(
                result.probabilities[x] >= result.probabilities[x - 1],
                "Probabilities should be monotonically increasing"
            );
        }
    }

    #[test]
    fn test_optimal_strategy_is_bold() {
        // With p < 0.5 (house edge), bold play is optimal:
        // bet min(x, T-x) to minimize rounds against unfavorable odds
        let result = solve_iterative(100, 0.49, 1e-12, 5000);
        assert_eq!(
            result.best_bets[50], 50,
            "At x=50, T=100: should bet $50 (bold play)"
        );
        assert_eq!(
            result.best_bets[25], 25,
            "At x=25, T=100: should bet $25 (bold play)"
        );
        assert_eq!(
            result.best_bets[10], 10,
            "At x=10, T=100: should bet $10 (bold play)"
        );
    }

    #[test]
    fn test_bold_play_probability() {
        // With bold play from x=50, T=100, p=0.49:
        // One bet: win → $100 (prob p), lose → $0 (prob 1-p)
        // So P(reaching $100 from $50) = p = 0.49
        let result = solve_iterative(100, 0.49, 1e-12, 5000);
        assert!(
            (result.probabilities[50] - 0.49).abs() < 1e-6,
            "r(50) with T=100, p=0.49 should be 0.49, got {}",
            result.probabilities[50]
        );
    }

    #[test]
    fn test_specific_values() {
        // Verify multiple states for T=100, p=0.49
        let result = solve_iterative(100, 0.49, 1e-12, 5000);

        // r(0) = 0 and r(T) = 1 always
        assert!((result.probabilities[0]).abs() < 1e-10);
        assert!((result.probabilities[100] - 1.0).abs() < 1e-10);

        // r(1) should be very small but positive
        assert!(result.probabilities[1] > 0.0);
        assert!(result.probabilities[1] < 0.02);

        // r(99) should be close to 1
        assert!(result.probabilities[99] > 0.85);

        // All probabilities between 0 and 1
        for x in 0..=100 {
            assert!(result.probabilities[x] >= 0.0 && result.probabilities[x] <= 1.0,
                "r({}) = {} out of [0,1]", x, result.probabilities[x]);
        }
    }

    #[test]
    fn test_favorable_odds_timid_is_optimal() {
        // With p > 0.5 (player has edge), timid play (bet $1) is optimal
        let result = solve_iterative(100, 0.51, 1e-12, 5000);
        assert_eq!(
            result.best_bets[50], 1,
            "At x=50, T=100, p=0.51: should bet $1 (timid play)"
        );
    }

    #[test]
    fn test_start_equals_target_minus_one() {
        // r(T-1) with bold play: bet $1, prob = p
        // But actually the optimal might be different. Just check it's high.
        let result = solve_iterative(100, 0.49, 1e-12, 5000);
        assert!(result.probabilities[99] > 0.85,
            "r(99) should be high, got {}", result.probabilities[99]);
    }

    #[test]
    fn test_small_target() {
        // T=10, p=0.49, start=5: bold play bets $5
        let result = solve_iterative(10, 0.49, 1e-12, 5000);
        assert_eq!(result.best_bets[5], 5);
        assert!((result.probabilities[5] - 0.49).abs() < 1e-6,
            "r(5) with T=10, p=0.49 should be 0.49, got {}", result.probabilities[5]);
    }

    #[test]
    fn test_recursion_does_not_work() {
        let iterative = solve_iterative(20, 0.49, 1e-12, 1000);
        let recursive = solve_recursive_BROKEN(20, 0.49);

        let mut found_diff = false;
        for x in 1..20 {
            if (iterative.probabilities[x] - recursive[x]).abs() > 0.01 {
                found_diff = true;
                break;
            }
        }
        assert!(
            found_diff,
            "Recursive approach does not work — results diverge from correct iterative solution"
        );
    }
}

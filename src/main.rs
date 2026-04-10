use algo_puzzles::problems::{egg_drop, gamblers_ruin};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("gambler") => run_gamblers_ruin(&args[2..]),
        Some("eggdrop") => run_egg_drop(&args[2..]),
        _ => {
            println!();
            println!("  ╔══════════════════════════════════════════════╗");
            println!("  ║       Algorithm Puzzles — Rust Edition       ║");
            println!("  ╠══════════════════════════════════════════════╣");
            println!("  ║                                              ║");
            println!("  ║  cargo run -- gambler [start] [target] [p]   ║");
            println!("  ║  cargo run -- eggdrop [eggs] [floors]        ║");
            println!("  ║                                              ║");
            println!("  ║  Examples:                                   ║");
            println!("  ║    cargo run -- gambler 1000 10000 0.49       ║");
            println!("  ║    cargo run -- eggdrop 2 100                 ║");
            println!("  ║                                              ║");
            println!("  ╚══════════════════════════════════════════════╝");
            println!();
        }
    }
}

fn run_gamblers_ruin(args: &[String]) {
    let start: usize = args.first().and_then(|s| s.parse().ok()).unwrap_or(1000);
    let target: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10000);
    let p_win: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.49);

    println!();
    println!("  Gambler's Ruin — Value Iteration");
    println!("  ─────────────────────────────────");
    println!("  Start:  ${}", start);
    println!("  Target: ${}", target);
    println!("  P(win): {}", p_win);
    println!();

    let result = gamblers_ruin::solve_iterative(target, p_win, 1e-12, 2000);

    let s = start.min(target);
    println!("  Converged in {} iterations", result.iterations);
    println!("  P(reaching ${} from ${}) = {:.8}", target, s, result.probabilities[s]);
    println!("  Optimal bet at ${}: ${}", s, result.best_bets[s]);

    println!();
    println!("  ┌──────────┬──────────────┬───────────┐");
    println!("  │  State   │ P(reaching)  │ Best Bet  │");
    println!("  ├──────────┼──────────────┼───────────┤");
    let step = target / 20;
    for i in 0..=20 {
        let x = (i * step).min(target);
        println!(
            "  │ ${:>6}  │  {:.8}  │  ${:>5}  │",
            x, result.probabilities[x], result.best_bets[x]
        );
    }
    println!("  └──────────┴──────────────┴───────────┘");
}

fn run_egg_drop(args: &[String]) {
    let eggs: usize = args.first().and_then(|s| s.parse().ok()).unwrap_or(2);
    let floors: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(100);

    println!();
    println!("  Egg Drop — Minimax DP");
    println!("  ─────────────────────");
    println!("  Eggs:   {}", eggs);
    println!("  Floors: {}", floors);
    println!();

    let t0 = std::time::Instant::now();
    let r_rec = egg_drop::solve_recursive(eggs, floors);
    let t_rec = t0.elapsed();

    let t0 = std::time::Instant::now();
    let r_iter = egg_drop::solve_iterative(eggs, floors);
    let t_iter = t0.elapsed();

    let t0 = std::time::Instant::now();
    let r_opt = egg_drop::solve_optimized(eggs, floors);
    let t_opt = t0.elapsed();

    println!("  ┌─────────────────────┬────────┬──────────────┐");
    println!("  │ Method              │ Answer │ Time         │");
    println!("  ├─────────────────────┼────────┼──────────────┤");
    println!("  │ Recursive + Memo    │ {:>6} │ {:>12?} │", r_rec.min_trials, t_rec);
    println!("  │ Iterative (O(kn²))  │ {:>6} │ {:>12?} │", r_iter.min_trials, t_iter);
    println!("  │ Optimized (O(knlgn))│ {:>6} │ {:>12?} │", r_opt.min_trials, t_opt);
    println!("  └─────────────────────┴────────┴──────────────┘");
    println!();

    assert_eq!(r_rec.min_trials, r_iter.min_trials);
    assert_eq!(r_iter.min_trials, r_opt.min_trials);
    println!("  All three methods agree: {} trials", r_iter.min_trials);

    // Print table
    let show_eggs = eggs.min(6);
    let show_floors = floors.min(20);
    println!();
    print!("  {:>4}", "");
    for n in 0..=show_floors {
        print!(" {:>3}", n);
    }
    if floors > show_floors { print!("  ..."); }
    println!();
    println!("  {}", "─".repeat(5 + 4 * (show_floors + 1)));
    for k in 1..=show_eggs {
        print!("  {:>2} │", k);
        for n in 0..=show_floors {
            print!(" {:>3}", r_iter.table[k][n]);
        }
        if floors > show_floors {
            print!("  ... {:>3}", r_iter.table[k][floors]);
        }
        println!();
    }
}

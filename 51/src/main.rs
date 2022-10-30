fn main() {
    const S: usize = 11;
    let t = std::time::Instant::now();
    let soln_to_eleven = lc_51_n_queens::Solution::solve_n_queens(S as i32).len();
    let duration = t.elapsed();
    println!("{S}-Queen takes {} ms to solve, yielding {soln_to_eleven} solutions", duration.as_millis());
}

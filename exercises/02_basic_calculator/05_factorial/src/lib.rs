// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!
fn factorial(n: u32) -> u32 {
    // Hide the inside function, as it attempts to make room for tail-call optimization by allowing
    // the "current" stack frame to be replaced by the next stack frame. As all required
    // information is passed into the next function call, the current frame doesn't have to kept in
    // memory (if the compiler allows) as such avoiding stack overflow.
    fn f(n: u32, fact: u32) -> u32 {
        match n {
            0 => fact,
            n => f(n - 1, fact * n),
        }
    }
    f(n, 1)
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}

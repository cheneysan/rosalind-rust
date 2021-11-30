/*
Rabbits and Recurrence Relations
https://rosalind.info/problems/fib/
*/

// def fib(n, k):
// a, b = 1, 1
// for i in range(2, n):
// a, b = b, k*a + b
// return b

fn fib(n: usize, k: usize) -> usize {
    let mut a;
    let mut b = 1;
    let mut c = 1;
    for _ in 2..n {
        a = b;
        b = c;
        c = b + (k * a);
    }
    c
}

#[cfg(test)]
mod tests {
    use crate::fib::fib;

    #[test]
    fn it_works() {
        let expected = 19;
        assert_eq!(expected, fib(5, 3));
    }

    #[test]
    fn actual_dataset() {
        println!("{}", fib(28, 4));
        // Correct answer: 66507086889
    }
}
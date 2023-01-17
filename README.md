# Assignment 1
Language used for this Assignment is Rust.

### How to run
Simply use `cargo run --release` command to run the code in the "release" mod. Release mod provides.
optimizations and therefore, code runs faster.

### About
The purpose of the assignment was to learn about multi-threading in the practical application of testing for primality.

The requirements were to find all primes in the specified range (100_000_000). The most time-consuming part of the algorithm
is simple primality check. No matter what kind of optimization one will be using, they will be stuck with O(sqrt(n)) for that operation.
The way the "primality test" works is by iteration through all numbers i = from 2 to sqrt(n), where n is the number to be tested and finding modulus between n and i.
If n can by divided by i and result in an integer (n % i == 0) then the number in question, n, is not a prime number.

That simple algorithm is good for small numbers but when we are talking about such big numbers as 100_000_000, it becomes clear.
that it will take quite some time to test all possible i variations. That's where multi-threading approach comes for help.

Essentially, each new number we are testing is independent from the previous or next number. As such, we can isolate primality test algorithm.
from the main algorithm for each new number by putting it on new thread. This optimization allows us to achieve faster execution because now we literally can check several numbers at the concurrently. Unfortunately, even this approach has its own limitations. The way concurrent programming works is platform depended.
however, in a nutshell, we can't simply spawn 1000 threads and expect 1000-fold faster execution of the algorithm because at some point, we will reach a plateau meaning 
the CPU will spend more time on switching between different threads to the point that it will not be feasible to use multi-threading anymore.   

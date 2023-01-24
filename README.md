# Assignment 1
Language used for this Assignment is Rust.

### How to run
Simply use `cargo run --release` command to run the code in the "release" mod. Release mod provides optimizations and therefore, code runs faster.

### About
The purpose of the assignment was to learn about multi-threading in the practical application of testing for primality.

The requirements were to find all primes in the specified range (100_000_000). The most time-consuming part of the algorithm is simple primality check. There are different algorithms for that, however, the most efficient one is usually a Sieve variation. For my project I used Sieve of Atkin implementation of the Sieve algorithm with addition of multi-threading capabilities. Any Sieve variation of the algorithm works in a way that essentially tries to reduce the "prime number space" by modifying the range of the numbers from 0 to upper_bound (which is 100m. in our case).

In Sieve of Atkin we use some math magic and since I barely know math, I can't really tell you anything more than that... However, almost any algorithm could be re-written using multi-threading, so that's what I did.

In this algorithm we have two loops and after careful observation, one can notice that the first loop is iterating through all numbers from 0 to upper_bound, while second loop applying some math magic to the number in question. As such, it becomes clear that the possible multi-threading optimization can be applied on the first loop... Essentially instead of "doing math magic" on one number, it would be great to do "math magic" on multiple numbers concurrently, and that's exactly what my implementation does.

When it comes to performance, there are several things to take into consideration. First, the algorithm itself is blazingly fast, even if it's ran on one thread. Because of that, it was hard for me to measure the performance different on range from 0 to 100 million... So, I made several runs with the upper_bound set to 1 billion...

The results of those runs are: 
- 1 Thread:
    - Average: 9.7 sec. 
- 8 Threads: 
    - Average: 7.4 sec. 
- 10 Threads: 
    - Average: 6.9 sec. 
- 14 Threads: 
    - Average: 7.5 sec.

As you might have noticed, more threads don’t necessarily mean faster code... In fact, there is a downside and limitations that one can hit fast when doing multi-threading programming. Specifically, what we can notice here is that the performance seems to degrade as more threads are being introduced. The reason for that consists of several parts:

1. When threads are created, it allocates virtual memory for its stack and private data structures... As such, the more threads we try to spawn, the more physical resources we will need to even run our code.
2. Another problem is in Time Slicing or Round Robing scheduling which works in a way that it gives every process an "equal opportunity" to use CPU... When we spawn too many threads, they begin to fight each other for accessing the real memory and thus, degrade performance.

Due to that, there is a point at which introducing more threads won't improve the performance but rather degrade it... Unless we are talking about some magical system that somehow only runs one process which will be our code (probably impossible to implement...).

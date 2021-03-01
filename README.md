# publicsuffix-benchmarks

Testing performance of public suffix list libraries.

## Results

On Intel(R) Core(TM) i5-8265U CPU @ 1.60GHz:

```bash
$ make -s

 Testing https://godoc.org/golang.org/x/net/publicsuffix
    1000 iterations = 0.001s
   10000 iterations = 0.013s
  100000 iterations = 0.099s
10000000 iterations = 9.946s

 Testing https://docs.rs/publicsuffix/2.0.2/publicsuffix/ (anycase)
    1000 iterations =   0.000s
   10000 iterations =   0.009s
  100000 iterations =   0.103s
10000000 iterations =   9.852s

 Testing https://docs.rs/publicsuffix/2.0.2/publicsuffix/
    1000 iterations =   0.000s
   10000 iterations =   0.004s
  100000 iterations =   0.043s
10000000 iterations =   4.166s

 Testing https://docs.rs/psl/2.0.3/psl/
    1000 iterations =   0.000s
   10000 iterations =   0.001s
  100000 iterations =   0.015s
10000000 iterations =   1.582s
```

# sumcheck
implementation of sumcheck protocol

features 
+ implements polynomial evalutation from scratch
+ Infers the number of variables inside a polynomial authomatically
+ Generates a prime that is larger than the number of variables

To be added
+ It requires powers to be inserted in a specific format x^2 as x*x
+ It requires variables to be numbered x'es...such as x1, x2...


To verify that it works for the given example in chapter 4.1 of the [book](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf), run the binary with the 
```
./path/to.binary x1*x1*x1*2+x1*x3+x2*x2 12
```




# average

Utility to average numbers given line by line on standard input, for use in shell pipelines.

```
average
compute average.
input is newline-delimted list of numbers from standard input.
Usage: average [options]
  --mean         compute mean (default)
  --median       compute median
  --mode         compute mode. Tiebreaker is not defined.
  
  --round [n]    number of decimal places to round to (default 5)
                 rounding applied only before final output for median and mean, 
                 but applied before attempting to compute mode.
  
  --int,    -i   round to the nearest integer (equiv. to --round 0)  
  --help,   -h   show this help then exit
  
  Example: $ printf '5\n2\n29\n' | average --median
           5
  
  ERRORS: Uses the laziest form of error handling available in Rust, 
  therefore error messages aren't very good.
  
  If you see the Rust runtime complaining about a "failed unwrap",
  It means invalid input was given.
```

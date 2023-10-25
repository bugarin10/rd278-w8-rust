# Comparison between rust and Python

In this repo two programs are implemented in rust and python. The programs run three nested loops and make an addition of the counters as follows:

```
Pseudo code:

declare empty variable sum

for i taking values from 0 to 1000:
    for j taking values from 0 to 1000:
        for k taking values from 0 to 1000:
            sum = sum  appending the sum: i + j + k
```

This code emulates a linear search in a 3D array. And the sum is only for the purpose of generate memory usage.

The results in Python are the following:

<img src="https://github.com/bugarin10/rd278-w8-rust/blob/main/static/python.png" />

And the results in Rust are the following:

<img src="https://github.com/bugarin10/rd278-w8-rust/blob/main/static/rust.png" />

As we can see Python used 30% more memory than rust, however, rust ran in a tenth of the time that python did.


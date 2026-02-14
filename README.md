# formats
this crate adds the `formats!()`, `prints!()`, and `printlns!()` macros, which work just like the standard `format!()`, `print!()`, and `println!()`, except they allow arbitrary expressions in placeholders. such as:

```rs
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

let s = formats!("the factorial of 5 is {factorial(5)}");
prints!("the size of i32 is {std::mem::size_of::<i32>()}");
printlns!("the first 10 numbers are {(0..10).collect::<Vec<i32>>():?}");
```


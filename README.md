# smart_format
this crate adds the `formats!()` macro, which works just like the standard `format!()`, except it allows arbitrary expressions in placeholders. such as:

```rs
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

println!("{}", formats!("the factorial of 5 is {factorial(5)}"));
println!("{}", formats!("the size of i32 is {std::mem::size_of::<i32>()}"));
println!("{}", formats!("the first 10 numbers are {(0..10).collect::<Vec<i32>>():?}"));
```


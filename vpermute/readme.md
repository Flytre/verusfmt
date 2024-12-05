How to run vpermute.

1. `cd` into the vpermute/src directory

2. Edit lines 62 and 63 lines of main.rs of vpermute, which is annotated CONFIGURE this:
```
    let mut visitors: Vec<&str> = vec!["FunctionInlineVisitor", "QuantifierVisitor"];
    let file_path: &str = "../examples/recur.rs";
```
This should include the list of visitors that should be permuted, and the target file. 

3. `cd ..`  to the top level vpermute directory and run `cargo run`

4. Results are found in the experiments folder in the verusfmt main directory.

[TODO]: add script to interperet results
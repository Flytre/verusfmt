How to run vpermute.

1. cd into the vpermute directory

2. edit the first 2 lines of main.rs of vpermute, which is annotated CONFIGURE this:
```
    let mut visitors: Vec<&str> = vec!["FunctionInlineVisitor", "QuantifierVisitor"];
    let file_path: &str = "../examples/recur.rs";
```

3. in the vpermute directory (or a subdirectory) run cargo run
4. check the experiments folder in the verusfmt main directory for output

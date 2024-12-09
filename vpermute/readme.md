<!-- How to run vpermute.

1. `cd` into the vpermute/src directory

2. Edit lines 62 and 63 lines of main.rs of vpermute, which is annotated CONFIGURE this:
```
    let mut visitors: Vec<&str> = vec!["FunctionInlineVisitor", "QuantifierVisitor"];
    let file_path: &str = "../examples/recur.rs";
```
This should include the list of visitors that should be permuted, and the target file. 

3. `cd ..`  to the top level vpermute directory and run `cargo run`

4. Results are found in the experiments folder in the verusfmt main directory.

[TODO]: add script to interperet results -->

### Running Visitor Permutation Experiments

1) Configure experiment config file in `./src/experiment_configs` or choose experiment config file
Ex:
```
            "name": "test",
            "program":  "../simple_end_to_end.rs", 
            "visitors": [
                "SimpleVisitor"
            ],
            "args": [
                "--debug"
            ]
``` 

**name**: name of experiment
**program**: relative path from vpermute root directory to testing file
**visitors**: list of visitors to run in permutation experiment
**args**: optional additional args (currently only supports `--debug` which add debug print statments)

2. Run experiment from vpermute root directory with the following command:
`python ./src/scripts/run_vpermute.py --config src/experiment_configs/[config].json`


3. Results will be found in `../permutation_experiments/`.
 Results include all permutations of applying the visitors to the input file, and a resutls .csv file. "diff_results.csv"
 diff_results.csv -- Includes the results of running `diff` between all possible pairs of permutations. 

_________________

#### Running vpermute directly

A single experiment can be run directly from the vpermute root directory using the following command:

`cargo run -- --file [relative_path_to_test_file] --experiment-name [name] --visitors [list_of visitors]`

optional args: `--debug`: prints debug statements

Ex:
`cargo run -- --file ../sample.rs --experiment-name sampleEx --visitors FunctionInlineVisitor,QuantifierVisitor,LoopVisitor --debug`


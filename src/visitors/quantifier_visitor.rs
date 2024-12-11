use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use regex::Regex;
// use std::collections::HashMap;
use std::collections::BTreeMap;


// Define a new struct for your custom visitor
pub struct QuantifierVisitor {
    _target_name: String, // Store target_name within QuantifierVisitor
}

impl QuantifierVisitor {
    pub fn new(target_name: String) -> Self {
        QuantifierVisitor { _target_name: target_name } // Return an instance of QuantifierVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("quantifier_expr", QuantifierVisitor::visit_quantifier); // contains logic for forall and exists
        handlers.insert(
            "quantifier_expr_no_struct",
            QuantifierVisitor::visit_quantifier,
        ); // treat the same as quantifier_expr
        handlers.insert("let_stmt", QuantifierVisitor::visit_let_stmt); // try to "catch" a choose quantifier before visiting it
        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(
            datum,
            pairs,
            &handler_map as &dyn HandlerInterface<CoreDatum>,
        );
    }

    fn visit_let_stmt(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // println!("Found stmt = {:?} - {}", pair.as_rule(), pair.as_str());

        if pair.as_str().contains("choose|") {
            let mut found_quantifier: Option<Pair<Rule>> = None; // Declare the variable to hold the quantifier
                                                                 // Logic for when the "choose|" substring is found
                                                                 // println!("The substring 'choose|' was found.");

            let mut stack = Vec::new(); // Stack to hold pairs to explore
            stack.push(pair.clone()); // Start with the original pair

            while let Some(current_pair) = stack.pop() {
                // Check if the current pair is a quantifier
                if current_pair.as_rule() == Rule::quantifier_expr {
                    println!("Found quantifier: {:?}", current_pair.as_str());
                    // Handle the quantifier as needed here (e.g., break, store it, etc.)
                    found_quantifier = Some(current_pair); // Assign the found quantifier
                    break; // Exit the loop once a quantifier is found
                }

                // If it's not a quantifier, push its inner pairs onto the stack
                for inner_pair in current_pair.into_inner() {
                    stack.push(inner_pair); // Add inner pair to the stack to explore
                }
            }
            // After the while loop, you can use found_quantifier
            if let Some(quantifier) = found_quantifier {
                // Do something with the quantifier, for example:
                // println!("Quantifier found and stored: {:?}", quantifier.as_str());

                //process choose!
                let mut inner_pairs = quantifier.clone().into_inner();

                if let Some(first_pair) = inner_pairs.next() {
                    // Initialize variables to hold closure parameters and expr
                    let mut closure_param_list = None;
                    let mut expr = None;

                    // Loop over inner elements to find `closure_param_list` and `expr`
                    for inner_pair in inner_pairs {
                        match inner_pair.as_rule() {
                            Rule::closure_param_list => {
                                closure_param_list = Some(inner_pair.clone());
                            }
                            Rule::expr => {
                                expr = Some(inner_pair.clone());
                            }
                            Rule::expr_no_struct => {
                                // treat the same as expr
                                expr = Some(inner_pair.clone());
                            }
                            _ => {}
                        }
                    }
		    let mut _var_name = "";
                    // Ensure both closure_param_list and expr were found
                    if let (Some(ref closure_param_list), Some(ref expr)) =
                        (closure_param_list, expr)
                    {
                        // Parse the closure_param_list to find variable names and types
                        let mut param_map: BTreeMap<String, String> = BTreeMap::new(); // Holds variable name and type

                        // Iterate over the inner pairs of closure_param_list
                        let mut quant_params = closure_param_list.clone().into_inner();
                        while let Some(quant_param_pair) = quant_params.next() {
                            match quant_param_pair.as_rule() {
                                Rule::param => {
                                    // println!("paramPair = {:?} :: {:?}",quant_param_pair.as_rule(), quant_param_pair.as_str());
                                    let mut inner_param_pairs =
                                        quant_param_pair.clone().into_inner();
                                    while let Some(inner_param_pair) = inner_param_pairs.next() {
                                        // println!("inner_param_pair = {:?} :: {:?}",inner_param_pair.as_rule(), inner_param_pair.as_str());
                                        match inner_param_pair.as_rule() {
                                            Rule::pat_no_top_alt => {
                                                _var_name = quant_param_pair.clone().as_str();
                                                // Split the string at ":"
                                                let parts: Vec<&str> = _var_name
                                                    .split(':')
                                                    .map(|part| part.trim())
                                                    .collect();

                                                // Ensure there are exactly two parts
                                                if parts.len() == 2 {
                                                    let key = parts[0];
                                                    let value = parts[1];

                                                    param_map
                                                        .insert(key.to_string(), value.to_string());
                                                } else {
                                                    println!("Variable format is incorrect.");
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        // Initialize a map for the variables in the expression
                        // let mut variables: HashMap<String, usize> = HashMap::new();
                        let mut variables: BTreeMap<String, usize> = BTreeMap::new();

                        for (key, _value) in &param_map {
                            variables.insert(key.to_string(), 0);
                            // println!("jere {:?}",key.to_string());
                        }
                        if first_pair.as_rule() == Rule::choose_str {
                            // println!("choose {:?} ", first_pair.as_str());
                            let satisfying_values_exists = Self::find_satisfying_values_exists(
                                &mut variables,
                                datum.finite_bound,
                            );
                            let mut expressions = Vec::new();
                            for s_val in satisfying_values_exists {
                                // Iterate over the key-value pairs in each map
                                let mut expr_with_values = expr.as_str().to_string();
                                for (variable_name, value) in s_val {
                                    // Create a regex pattern for exact match of the variable name
                                    let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                                    let regex = Regex::new(&pattern).unwrap();

                                    // Replace only exact matches in the expression
                                    expr_with_values = regex
                                        .replace_all(&expr_with_values, &value.to_string())
                                        .into_owned();
                                }
                                expressions.push(expr_with_values);
                            }

                            let mut iter = expressions.clone().into_iter().peekable();
                            if let Some(first_expr) = iter.next() {
                                datum
                                    .program_mut()
                                    .push_str(&format!("assert(({})", first_expr.as_str()));
                                for expanded_expr in iter {
                                    datum
                                        .program_mut()
                                        .push_str(&format!("|| ({})", expanded_expr.as_str()));
                                }
                            }
                            datum.program_mut().push_str(");");
                            // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                        }
                    }
                }
            } else {
                println!("No quantifier was found.");
            }
        } else {
            // println!("The substring 'choose|' was not found.");
        }

        // Continue with further processing if necessary
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_quantifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        println!(
            "Found quantifier = {:?} - {}",
            pair.as_rule(),
            pair.as_str()
        );
        // common preamble
        let mut inner_pairs = pair.clone().into_inner();

        if let Some(first_pair) = inner_pairs.next() {
            // Initialize variables to hold closure parameters and expr
            let mut closure_param_list = None;
            let mut expr = None;

            // Loop over inner elements to find `closure_param_list` and `expr`
            for inner_pair in inner_pairs {
                match inner_pair.as_rule() {
                    Rule::closure_param_list => {
                        closure_param_list = Some(inner_pair.clone());
                    }
                    Rule::expr => {
                        expr = Some(inner_pair.clone());
                    }
                    Rule::expr_no_struct => {
                        // treat the same as expr
                        expr = Some(inner_pair.clone());
                    }
                    _ => {}
                }
            }
            // Ensure both closure_param_list and expr were found
            if let (Some(ref closure_param_list), Some(ref expr)) = (closure_param_list, expr) {
                // Parse the closure_param_list to find variable names and types
                // let mut param_map: HashMap<String, String> = HashMap::new(); // Holds variable name and type
                let mut param_map: BTreeMap<String, String> = BTreeMap::new();


                // Iterate over the inner pairs of closure_param_list
                let mut quant_params = closure_param_list.clone().into_inner();
                while let Some(quant_param_pair) = quant_params.next() {
                    match quant_param_pair.as_rule() {
                        Rule::param => {
                            // println!("paramPair = {:?} :: {:?}",quant_param_pair.as_rule(), quant_param_pair.as_str());
                            let mut inner_param_pairs = quant_param_pair.clone().into_inner();
                            let mut _var_name = "";
                            while let Some(inner_param_pair) = inner_param_pairs.next() {
                                // println!("inner_param_pair = {:?} :: {:?}",inner_param_pair.as_rule(), inner_param_pair.as_str());
                                match inner_param_pair.as_rule() {
                                    Rule::pat_no_top_alt => {
                                        _var_name = quant_param_pair.clone().as_str();
                                        // Split the string at ":"
                                        let parts: Vec<&str> =
                                            _var_name.split(':').map(|part| part.trim()).collect();

                                        // Ensure there are exactly two parts
                                        if parts.len() == 2 {
                                            let key = parts[0];
                                            let value = parts[1];

                                            param_map.insert(key.to_string(), value.to_string());
                                        } else {
                                            println!("Variable format is incorrect.");
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // Initialize a map for the variables in the expression

                let mut variables: BTreeMap<String, usize> = BTreeMap::new();
                for (key, _value) in &param_map {
                    variables.insert(key.to_string(), 0);
                }
                
                if first_pair.as_rule() == Rule::forall_str {
                    // Split implication
                    let (lhs_opt, rhs_opt) = VerusParser::split_implication(expr.as_str());
                    if let (Some(lhs_expr), Some(rhs_expr)) = (lhs_opt, rhs_opt) {
                        // println!("lhs = {:?}",lhsExpr.as_str());
                        // println!("RHS: {:?}", rhsExpr.as_str());
                        // println!("Parameters map: {:?}", param_map);

                        let expr = lhs_expr.as_str();

                        let (all_found, missing_vars) =
                            Self::check_missing_variables_from_expr(expr, &variables);

                        //TODO Clean-up code
                        if all_found {
                            let (lower_bound, upper_bound) = Self::find_bounds(expr); // Find both bounds
                            println!(
                                "Upper bound found: {},lower bound Found {}. {:?}",
                                upper_bound, lower_bound, &mut variables
                            );
                            // Call the function to find all satisfying combinations
                            let mut _satisfying_values: Option<Vec<BTreeMap<String, usize>>> = None;
                            if Self::can_be_evaluated_by_eval_forall_bounds(expr, &variables) {
                                println!(
                                    "Expression '{}' CAN be evaluated by evalForallBounds",
                                    expr
                                );

                                // let (lower_bound, upper_bound) = Self::find_bounds(expr);
                                // let satisfying_values = Self::find_satisfying_values(expr, &mut variables, lower_bound, upper_bound);
                                // Process satisfying values
                                _satisfying_values = Some(Self::find_satisfying_values(
                                    expr,
                                    &mut variables,
                                    lower_bound,
                                    upper_bound,
                                ));
                            } else {
                                println!(
                                    "Expression '{}' cannot be evaluated by evalForallBounds",
                                    expr
                                );
                                _satisfying_values = Some(Self::generate_combinations_simple(
                                    &mut variables,
                                    datum.finite_bound,
                                ));
                                // println!("Satisfying combinations:");
                                // for combination in satisfying_values.clone() {
                                //     println!("{:?}", combination);
                                // }

                                if let Some(satisfying_values) = _satisfying_values {
                                    // Collect pairs of (LHS expression, RHS expression)
                                    let mut generated_expressions = Vec::new();

                                    for s_val in satisfying_values {
                                        // Generate LHS with values
                                        let mut lhs_with_values = lhs_expr.as_str().to_string();
                                        let mut rhs_with_values = rhs_expr.as_str().to_string();

                                        // Replace variables in both LHS and RHS
                                        for (variable_name, value) in s_val {
                                            let pattern =
                                                format!(r"\b{}\b", regex::escape(&variable_name));
                                            let regex = Regex::new(&pattern).unwrap();

                                            lhs_with_values = regex
                                                .replace_all(&lhs_with_values, &value.to_string())
                                                .into_owned();
                                            rhs_with_values = regex
                                                .replace_all(&rhs_with_values, &value.to_string())
                                                .into_owned();
                                        }

                                        generated_expressions
                                            .push((lhs_with_values, rhs_with_values));
                                    }

                                    // println!("Generated (LHS, RHS) pairs:");
                                    // for (lhs, rhs) in &generated_expressions {
                                    //     println!("LHS: {}, RHS: {}", lhs, rhs);
                                    // }
                                    let mut iter =
                                        generated_expressions.clone().into_iter().peekable();
                                    if let Some((first_lhs, first_rhs)) = iter.next() {
                                        datum.program_mut().push_str(&format!(
                                            "(({}) ==> ({}))",
                                            first_lhs, first_rhs
                                        ));
                                        for (expanded_lhs, expanded_rhs) in iter {
                                            datum.program_mut().push_str(&format!(
                                                " && (({}) ==> ({}))",
                                                expanded_lhs, expanded_rhs
                                            ));
                                        }
                                    }
                                }

                                return;
                            }

                            // Print out the satisfying combinations
                            // println!("Satisfying combinations:");
                            // for combination in satisfying_values.clone() {
                            //     println!("{:?}", combination);
                            // }

                            let mut expressions = Vec::new();
                            if let Some(satisfying_values) = _satisfying_values {
                                for s_val in satisfying_values {
                                    // Iterate over the key-value pairs in each map
                                    let mut expr_with_values = rhs_expr.as_str().to_string();
                                    println!("rhs = {:?}", s_val);
                                    for (variable_name, value) in s_val {
                                        // Replace each variable in the RHS with its value
                                        let pattern =
                                            format!(r"\b{}\b", regex::escape(&variable_name));
                                        let regex = Regex::new(&pattern).unwrap();

                                        // expr_with_values = expr_with_values.replace(&variable_name, &value.to_string());
                                        expr_with_values = regex
                                            .replace_all(&expr_with_values, &value.to_string())
                                            .into_owned();
                                    }
                                    expressions.push(expr_with_values);
                                }
                            }

                            let mut iter = expressions.clone().into_iter().peekable();

                            if let Some(first_expr) = iter.next() {
                                datum.program_mut().push_str(first_expr.as_str());
                                for expanded_rhs_expr in iter {
                                    datum
                                        .program_mut()
                                        .push_str(&format!("&& {}", expanded_rhs_expr.as_str()));
                                }
                            }
                        } else {
                            // upper bound of LHS is not concrete, so do nothing - for now!
                            println!(
                                "No concrete Upper bound, but the passed in bound is: {}, {:?}",
                                datum.finite_bound, missing_vars
                            );
                            let (upper_bound, _lower_bound) = Self::find_bounds(expr); // Find upper bound, lower may be defaulted
                                                                                      // Since lower bound is likely not determined, handle this case accordingly
                            println!("Lower bound found: {}, And Upper bound is not applicable in this context.", upper_bound);

                            if missing_vars.len() == 1 {
                                //[assumption for now]
                                let missing_var = &missing_vars[0]; // Get the missing variable

                                // Extract the constants in the expression for the lower bound
                                let mut lower_bound = 0;
                                let parts: Vec<&str> = expr.split_whitespace().collect(); // Assuming `expr` is available in this context

                                for part in &parts {
                                    if let Ok(val) = part.parse::<usize>() {
                                        lower_bound = val;
                                        break; // Stop after finding the first constant
                                    }
                                }

                                // Store the generated expressions
                                let mut generated_expressions = Vec::new();

                                // Generate expressions for the missing variable
                                for val in lower_bound..=datum.finite_bound {
                                    // Generate the base expression for the missing variable
                                    let base_expression =
                                        format!("{} == {} ==> ", missing_var, val);
                                    let modified_expr = expr.replace(missing_var, &val.to_string());

                                    // Call the function to find all satisfying combinations
                                    // println!("try to find sats {:?} , {:?} , {:?} , {:?}", modified_expr, variables,upper_bound, datum.finite_bound);
                                    let satisfying_values = Self::find_satisfying_values(
                                        &modified_expr,
                                        &mut variables,
                                        upper_bound,
                                        datum.finite_bound,
                                    );
                                    // println!("Satisfying combinations:");
                                    // for combination in &satisfying_values {
                                    //     println!("{:?}", combination);
                                    // }
                                    // Prepare a string to concatenate all satisfying expressions
                                    let mut rhs_expressions = Vec::new();

                                    // Generate RHS expressions based on satisfying values
                                    for s_val in satisfying_values {
                                        let mut expr_with_values = rhs_expr.as_str().to_string();
                                        for (variable_name, value) in s_val {
                                            let pattern =
                                                format!(r"\b{}\b", regex::escape(&variable_name));
                                            let regex = Regex::new(&pattern).unwrap();
                                            expr_with_values = regex
                                                .replace_all(&expr_with_values, &value.to_string())
                                                .into_owned();
                                        }
                                        rhs_expressions.push(expr_with_values);
                                    }

                                    // Check if rhs_expressions is empty, and if so, add "true"
                                    if rhs_expressions.is_empty() {
                                        rhs_expressions.push("true".to_string());
                                    }

                                    // Concatenate all RHS expressions into a single string
                                    let rhs_concat = rhs_expressions.join(" && "); // Concatenate with "&&" or any desired separator

                                    // Create the final expression
                                    let final_expression =
                                        format!("({}({}))", base_expression, rhs_concat);
                                    generated_expressions.push(final_expression);
                                    // Add to the generated expressions
                                }

                                // for expr in &generated_expressions {
                                //     println!("{}", expr);
                                // }
                                let mut iter = generated_expressions.clone().into_iter().peekable();

                                if let Some(first_expr) = iter.next() {
                                    datum.program_mut().push_str(first_expr.as_str());
                                    for expanded_rhs_expr in iter {
                                        datum.program_mut().push_str(&format!(
                                            "&& {}",
                                            expanded_rhs_expr.as_str()
                                        ));
                                    }
                                }

                                // Continue with further processing as necessary
                                // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                            } else {
                                VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                            }
                        }
                    }
                    // end forall
                } else if first_pair.as_rule() == Rule::exists_str {
                    // println!("found exists expr  {}", first_pair);
                    // println!("expr and closure = {:?} :: {:?}",expr.as_str(), closure_param_list.as_str());
                    let satisfying_values_exists =
                        Self::find_satisfying_values_exists(&mut variables, datum.finite_bound);

                    let mut expressions = Vec::new();

                    for s_val in satisfying_values_exists {
                        // Iterate over the key-value pairs in each map
                        let mut expr_with_values = expr.as_str().to_string();
                        for (variable_name, value) in s_val {
                            // Create a regex pattern for exact match of the variable name
                            let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                            let regex = Regex::new(&pattern).unwrap();

                            // Replace only exact matches in the expression
                            expr_with_values = regex
                                .replace_all(&expr_with_values, &value.to_string())
                                .into_owned();
                        }
                        expressions.push(expr_with_values);
                    }

                    let mut iter = expressions.clone().into_iter().peekable();
                    if let Some(first_expr) = iter.next() {
                        datum
                            .program_mut()
                            .push_str(&format!("({})", first_expr.as_str()));
                        for expanded_expr in iter {
                            datum
                                .program_mut()
                                .push_str(&format!("|| ({})", expanded_expr.as_str()));
                        }
                    }
                    // end exists
                } else {
                    VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                }
            }
        }
    }

    // -------------------------
    // ---- HELPER FUNCTIONS ---
    // -------------------------

    fn generate_combinations_simple(
        variables: &BTreeMap<String, usize>,
        bound: usize,
    ) -> Vec<BTreeMap<String, usize>> {
        let var_names: Vec<String> = variables.keys().cloned().collect();
        let mut results = Vec::new();
        let mut current_values = vec![0; var_names.len()];

        loop {
            // Create a new BTreeMap for the current combination
            let mut combination = BTreeMap::new();
            for (i, var_name) in var_names.iter().enumerate() {
                combination.insert(var_name.clone(), current_values[i]);
            }
            results.push(combination);

            // Increment the combination to the next one
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < bound {
                    current_values[idx] += 1;
                    break;
                } else {
                    current_values[idx] = 0;
                    idx += 1;
                }
            }

            // If we have exhausted all combinations, break the loop
            if idx == current_values.len() {
                break;
            }
        }

        results
    }

    fn find_bounds(expr: &str) -> (usize, usize) {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        let mut lower_bound: Option<usize> = None; // Use Option to handle uninitialized state
        let mut upper_bound: Option<usize> = None;

        // Iterate over the parts to find numeric literals
        for part in &parts {
            if let Ok(val) = part.parse::<usize>() {
                // Update the lower and upper bounds accordingly
                if lower_bound.is_none() || val < lower_bound.unwrap() {
                    lower_bound = Some(val);
                }
                if upper_bound.is_none() || val > upper_bound.unwrap() {
                    upper_bound = Some(val);
                }
            }
        }

        // Return defaults if bounds were never set
        (
            lower_bound.unwrap_or(0), // Default to 0 if no lower bound was found
            upper_bound.unwrap_or(usize::MAX), // Default to max value if no upper bound was found
        )
    }

    fn find_satisfying_values_exists(
        variables: &mut BTreeMap<String, usize>,
        bound: usize,
    ) -> Vec<BTreeMap<String, usize>> {
        let mut satisfying_combinations = Vec::new();

        // Get the variable names
        let variable_names: Vec<String> = variables.keys().cloned().collect();

        // Generate all combinations within the range for each variable
        let mut current_values = vec![0; variable_names.len()];

        loop {
            // Set each variable in the BTreeMap to its current value in the combination
            for (i, var_name) in variable_names.iter().enumerate() {
                variables.insert(var_name.clone(), current_values[i]);
            }

            // Add the current combination to the satisfying_combinations list
            satisfying_combinations.push(variables.clone());

            // Move to the next combination of values
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < bound {
                    current_values[idx] += 1;
                    break;
                } else {
                    current_values[idx] = 0;
                    idx += 1;
                }
            }

            if idx == current_values.len() {
                break; // Exit the loop when all combinations have been generated
            }
        }

        satisfying_combinations
    }

    fn check_missing_variables_from_expr(
        expr: &str,
        variables: &BTreeMap<String, usize>,
    ) -> (bool, Vec<String>) {
        // Split the expression by whitespace
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Operators to ignore
        let operators = ["<", "<=", ">", ">=", "==", "!=", "&&", "||"];
        let mut missing_variables = Vec::new(); // Vector to hold missing variable names

        for part in parts {
            // Skip operators and numeric literals
            if operators.contains(&part) || part.parse::<usize>().is_ok() || part == "bound" {
                continue;
            }

            // Check if `part` is a variable not found in `variables`
            if !variables.contains_key(part) {
                // If it's missing, add to the list of missing variables
                missing_variables.push(part.to_string());
            }
        }

        // Return a tuple: (true if no missing variables, false otherwise, and the list of missing variables)
        (missing_variables.is_empty(), missing_variables)
    }

    //currently supports expressions of the following form:
    // Constant < x < Bound
    // Constant < x < y < Bound
    // With comparison operators {< , <= , >, >=}
    fn eval_forall_bounds(
        expr: &str,
        variables: &BTreeMap<String, usize>,
        lower_bound: usize,
        upper_bound: usize,
    ) -> bool {
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Start with the first value or variable in the chain
        let mut last_value = match parts[0].parse::<usize>() {
            Ok(val) => val,                                   // A numeric literal
            Err(_) => *variables.get(parts[0]).unwrap_or(&0), // Variable from param_map
        };

        let mut idx = 1;
        while idx < parts.len() - 1 {
            // Check for valid operator
            let operator = parts[idx];
            let next_value = match parts[idx + 1] {
                var_or_val => match var_or_val.parse::<usize>() {
                    Ok(val) => val, // Numeric literal
                    Err(_) => match variables.get(var_or_val) {
                        Some(&val) => val, // Variable from param_map
                        None => {
                            return false; // Handle the missing variable case
                        }
                    },
                },
            };

            // Compare based on the operator
            let comparison_holds = match operator {
                "<" => last_value < next_value,
                "<=" => last_value <= next_value,
                ">" => last_value > next_value,
                ">=" => last_value >= next_value,
                _ => {
                    println!("Unexpected operator: {}", operator);
                    return false;
                }
            };

            if !comparison_holds {
                return false;
            }

            // Move to the next value in the chain
            last_value = next_value;
            idx += 2;
        }

        // Ensure last_value is within the bounds
        last_value >= lower_bound && last_value <= upper_bound
    }

    fn can_be_evaluated_by_eval_forall_bounds(
        expr: &str,
        variables: &BTreeMap<String, usize>,
    ) -> bool {
        // Tokenize the expression by splitting on whitespace
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Ensure there are enough tokens for a valid comparison chain
        if parts.len() < 3 || parts.len() % 2 == 0 {
            return false;
        }

        // Check the pattern: value/operator/value/operator/...
        for (i, part) in parts.iter().enumerate() {
            if i % 2 == 0 {
                // Expect a variable or numeric literal
                if part.parse::<usize>().is_err() && !variables.contains_key(*part) {
                    return false; // Invalid variable or missing numeric literal
                }
            } else {
                // Expect a valid comparison operator
                if !["<", "<=", ">", ">="].contains(part) {
                    return false; // Unsupported operator
                }
            }
        }

        true
    }

    fn find_satisfying_values(
        expr: &str,
        variables: &mut BTreeMap<String, usize>,
        lower_bound: usize,
        upper_bound: usize,
    ) -> Vec<BTreeMap<String, usize>> {
        let mut satisfying_combinations = Vec::new();
        let _parts: Vec<&str> = expr.split_whitespace().collect();

        // Gather variable names from the expression
        let variable_names: Vec<String> = variables.keys().cloned().collect();

        // Generate all combinations within the range and evaluate them
        let mut current_values = vec![lower_bound; variable_names.len()];
        loop {
            // Set variable values in the BTreeMap
            for (i, var_name) in variable_names.iter().enumerate() {
                variables.insert(var_name.clone(), current_values[i]);
            }

            // Evaluate the expression with the current combination
            if Self::eval_forall_bounds(expr, variables, lower_bound, upper_bound) {
                satisfying_combinations.push(variables.clone());
            }

            // Increment to the next combination
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < upper_bound {
                    current_values[idx] += 1;
                    break;
                } else {
                    current_values[idx] = lower_bound;
                    idx += 1;
                }
            }

            if idx == current_values.len() {
                break; // We’ve exhausted all combinations
            }
        }

        satisfying_combinations
    }
}

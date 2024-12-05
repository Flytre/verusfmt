use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
    static ref PARENT_FUNCTION_PARAM_MAP: Mutex<HashMap<String, HashMap<String, String>>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_LET_TYPE_MAP: Mutex<HashMap<String, HashMap<String, String>>> =
        Mutex::new(HashMap::new());
}
// Define a new struct for your custom visitor
pub struct SetSubsetVisitor {
    _target_name: String, // Store target_name within SetSubsetVisitor
}

impl SetSubsetVisitor {
    pub fn new(target_name: String) -> Self {
        SetSubsetVisitor { _target_name: target_name } // Return an instance of SetSubsetVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("expr", SetSubsetVisitor::visit_expr);
        handlers.insert("fn", SetSubsetVisitor::visit_function);
        handlers.insert("let_stmt", SetSubsetVisitor::visit_let_stmt);

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
        let mut inner_pairs = pair.clone().into_inner();

        let mut receiving_var: Option<String> = None;
        let mut type_str: Option<String> = None;
        let mut let_param_map = HashMap::new();

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::pat => {
                    let pat_pair = inner_pair.clone();
                    receiving_var = Some(pat_pair.as_str().to_string());
                }
                Rule::r#type => {
                    let type_pair = inner_pair.clone();
                    type_str = Some(type_pair.as_str().to_string());
                }
                _ => {}
            }
        }
        if let (Some(receiving_var), Some(type_str)) = (receiving_var.clone(), type_str.clone()) {
            //   println!("Let Inner = {:?} {:?}", receiving_var.as_str(), type_str.as_str());
            let_param_map.insert(receiving_var, type_str);
        }
        let parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();

        {
            let mut parent_let_param_map = PARENT_FUNCTION_LET_TYPE_MAP.lock().unwrap();
            parent_let_param_map
                .entry(parent_name.clone())
                .and_modify(|existing_map| {
                    existing_map.extend(let_param_map.clone()); // Add or overwrite existing keys
                })
                .or_insert_with(|| let_param_map.clone()); // Insert new entry if not present
        }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut param_list = None;
        let mut param_map = HashMap::new();

        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::param_list => {
                    param_list = Some(inner_pair.clone());
                }
                _ => {}
            }
        }

        let mut current_param_name = None;
        if let Some(ref param_list) = param_list {
            for inner_param in param_list.clone().into_inner() {
                for param_vals in inner_param.clone().into_inner() {
                    let rule_str = format!("{:?}", param_vals.as_rule());

                    match rule_str.as_str() {
                        "pat_no_top_alt" => {
                            current_param_name = Some(param_vals.as_str().to_string());
                        }
                        "type" => {
                            if let Some(param_name) = current_param_name.take() {
                                param_map.insert(param_name, param_vals.as_str().to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        {
            let mut parent_param_map = PARENT_FUNCTION_PARAM_MAP.lock().unwrap();
            parent_param_map.insert(name.to_string(), param_map.clone());
        }
        // for (param, param_type) in &param_map {
        //     println!("Parameter: {}, Type: {}", param, param_type);
        // }

        {
            let mut parent_name = PARENT_FUNCTION_NAME.lock().unwrap();
            *parent_name = name.to_string();
        }

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
    fn find_subset_sets(pair: &Pair<Rule>) -> (Option<String>, Option<String>) {
        // Check if the current level contains a Rule::name_ref
        let has_name_ref = pair
            .clone()
            .into_inner()
            .any(|p| p.as_rule() == Rule::name_ref);

        if has_name_ref {
            let mut left_set = None;
            let mut right_set = None;

            for inner_pair in pair.clone().into_inner() {
                match inner_pair.as_rule() {
                    Rule::name_ref => {
                        // Skip this case since we're only using `name_ref` to detect the level.
                    }
                    Rule::expr_inner => {
                        // Assume the left-hand set
                        left_set = Some(inner_pair.as_str().to_string());
                    }
                    Rule::arg_list => {
                        // Assume the right-hand set
                        for inner_arg in inner_pair.clone().into_inner() {
                            if let Rule::comma_delimited_exprs = inner_arg.as_rule() {
                                right_set = Some(inner_arg.as_str().to_string());
                            }
                        }
                    }
                    _ => {}
                }

                // Stop searching if both sets are found
                if left_set.is_some() && right_set.is_some() {
                    break;
                }
            }

            return (left_set, right_set);
        } else {
            // Recurse into the AST if no Rule::name_ref is found
            for inner_pair in pair.clone().into_inner() {
                let result = Self::find_subset_sets(&inner_pair);
                if result.0.is_some() && result.1.is_some() {
                    return result;
                }
            }
        }

        // If no result is found, return None for both sets
        (None, None)
    }

    fn visit_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        if pair.as_str().contains("subset_of") && pair.as_str().contains("assert") {
            let (left_set, right_set) = Self::find_subset_sets(&pair);

            if let (Some(left), Some(right)) = (left_set, right_set) {
                // println!("found l and r {:?} :: {:?}", left, right);

                // Retrieve `parent_params` and `parent_let_params`
                let parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();
                let parent_params = {
                    let parent_map = PARENT_FUNCTION_PARAM_MAP.lock().unwrap().clone();
                    parent_map.get(&parent_name).cloned()
                };

                let parent_let_params = {
                    let parent_let_map = PARENT_FUNCTION_LET_TYPE_MAP.lock().unwrap().clone();
                    parent_let_map.get(&parent_name).cloned()
                };

                // Define numerical types
                let numerical_types = vec![
                    "int", "nat", "usize", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
                ];

                // Helper function to check if a variable is a Set<numeric type>
                let is_set_of_numeric =
                    |var_name: &str, params_map: &Option<HashMap<String, String>>| {
                        params_map.as_ref().and_then(|params| {
                            params.get(var_name).and_then(|var_type| {
                                // Trim the type and check for `Set<numeric_type>`
                                let trimmed_type = var_type.trim();
                                trimmed_type
                                    .strip_prefix("Set<")
                                    .and_then(|inner_type| inner_type.strip_suffix('>'))
                                    .map(|inner_type| numerical_types.contains(&inner_type))
                            })
                        }) == Some(true)
                    };

                // Check types for `left` and `right`
                let left_is_valid = is_set_of_numeric(&left, &parent_params)
                    || is_set_of_numeric(&left, &parent_let_params);
                let right_is_valid = is_set_of_numeric(&right, &parent_params)
                    || is_set_of_numeric(&right, &parent_let_params);

                if left_is_valid && right_is_valid {
                    println!("Both left and right are valid Set<numeric type>.");
                    // Generate the required assertions
                    let mut unwraped_code = format!(
                        "let mut {left}_mut: Set<int> = {left};\n\
                         let mut {right}_mut: Set<int> = {right};\n\
                         assert({left}_mut.finite());\n\
                         assert({right}_mut.finite());\n\
                         assert({right}_mut.len() >= {left}_mut.len());\n",
                        left = left,
                        right = right,
                    );

                    // Generate the repeated block for datum.finite_bound times
                    for _ in 0..datum.finite_bound {
                        unwraped_code.push_str(&format!(
                            "if ({left}_mut.len() == 0) {{\n\
                                assert({left}_mut.subset_of({right}_mut));\n\
                            }}\n\
                            let {left}_val = {left}_mut.choose();\n\
                            if (!{right}_mut.contains({left}_val)) {{\n\
                                assert(!{left}_mut.subset_of({right}_mut));\n\
                            }} else {{\n\
                                let {right}_removed = {right}_mut.remove({left}_val);\n\
                                let {left}_removed = {left}_mut.remove({left}_val);\n\
                                assert({right}_removed.len() == {right}_mut.len() - 1);\n\
                                assert({left}_removed.len() == {left}_mut.len() - 1);\n\
                                {right}_mut = {right}_removed;\n\
                                {left}_mut = {left}_removed;\n\
                            }}\n",
                            left = left,
                            right = right,
                        ));
                    }

                    datum
                        .program_mut()
                        .push_str(&format!("{}\n{}", unwraped_code, pair.as_str()));
                    // Additional logic for valid sets can go here
                } else {
                    println!(
                        "Invalid types: left: {:?}, right: {:?}",
                        parent_params, parent_let_params
                    );
                }
            } else {
                println!("Could not find both left and right sets.");
            }
        } else {
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
    }
}

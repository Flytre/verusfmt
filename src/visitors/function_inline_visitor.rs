use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::ParseAndFormatError;
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashSet;
use std::collections::HashMap;


lazy_static! {
    static ref HANDLED_INLINE_FNS: Mutex<HashSet<String>> =
        Mutex::new(HashSet::new());
}


pub fn find<'a>(pair: &'a Pair<'a, Rule>, target: Rule) -> Vec<Pair<'a, Rule>> {
    let mut matches = Vec::new();
    recursive_find(pair.clone(), target, &mut matches);
    matches
}

fn recursive_find<'a>(pair: Pair<'a, Rule>, target: Rule, matches: &mut Vec<Pair<'a, Rule>>) {
    if pair.as_rule() == target {
        matches.push(pair.clone());
    }

    for inner_pair in pair.into_inner() {
        recursive_find(inner_pair, target, matches);
    }
}

pub struct FunctionInlineVisitor {}

impl FunctionInlineVisitor {
    fn create_combined_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();

        handlers.insert("fn", FunctionInlineVisitor::visit_function);
        handlers.insert("expr", FunctionInlineVisitor::visit_expr);
        handlers.insert(
            "verus_macro_use",
            FunctionInlineVisitor::visit_verus_macro_use,
        );
        handlers.insert("let_stmt", FunctionInlineVisitor::visit_let_stmt);
        handlers.insert("fn_block_expr", FunctionInlineVisitor::visit_fn_block_expr);
        handlers
    }

    pub fn visit_all(datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = FunctionInlineVisitor::create_combined_handler_map();
        VerusVisitor::visit_all(
            datum,
            pairs,
            &handler_map as &dyn HandlerInterface<CoreDatum>,
        );
    }

    /// Handler for the "fn" rule. This is a Core-specific handler.
    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();
        println!("Stored the function body for {}", name);
        datum
            .fn_map
            .insert(name.to_string(), pair.as_str().to_string());
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_let_stmt(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut identifier: Option<String> = None;
        let mut value: Option<String> = None;
        let mut found_eq = false;

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::pat => {
                    let identifiers = find(&inner_pair, Rule::identifier);
                    if identifiers.len() == 1 {
                        identifier = Some(identifiers[0].clone().as_str().to_string());
                    }
                }
                Rule::eq_str => {
                    found_eq = true;
                }
                Rule::expr if found_eq => {
                    let literals = find(&inner_pair, Rule::int_number);
                    if literals.len() == 1 {
                        value = Some(literals[0].clone().as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        if let (Some(identifier), Some(value)) = (identifier.as_ref(), value.as_ref()) {
            println!(
                "Found variable value {} = {}",
                identifier.clone(),
                value.clone()
            );
            datum.variable_map.insert(identifier.clone(), value.clone());
            datum
                .variable_stack
                .last_mut()
                .unwrap()
                .push(identifier.clone());
        }
        VerusVisitor::default_visit(datum, pair, handlers);
    }

    fn visit_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut inner_pairs = pair.clone().into_inner();

        let mut function_name: Option<String> = None;
        let mut arguments: Option<String> = None;
        let mut nested_exprs_list = Vec::new(); // Collect nested pairs that may follow i.e. expr && nested_expr

        let mut prev: bool = false;

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    let nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        if !(function_pair.as_str().contains("::")) {
                            function_name = Some(function_pair.as_str().to_string());
                        }
                        prev = true;
                    }
                }
                Rule::arg_list => {
                    let args: String = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect();
                    if prev {
                        arguments = Some(args.clone());
                    }
                }
                _ => {
                    prev = false;
                    nested_exprs_list.push(inner_pair);

                }
            }
        }

        let mut handled = false;
        if let (Some(function_name), Some(arguments)) = (function_name, arguments) {
            println!(
                "Function called: {} with args: {:?}",
                function_name, arguments
            );
            let mut args: Vec<String> = arguments
                .as_str()
                .split(',')
                .map(|s| {
                    let trimmed = s.trim().to_string();
                    datum.variable_map.get(&trimmed).cloned().unwrap_or(trimmed)
                })
                .collect();
            // helps remove additional arg if Function Inline Visitor is called
            // more than once, leading to formatting issues that add "\n" to args
            if args.len() > 1 && args.last().map_or(false, |s| s.is_empty()) {
                args.pop();
            }
            datum
                .fn_calls
                .entry(function_name.clone())
                .or_insert_with(Vec::new)
                .push(args.clone());


            if args.iter().all(|arg| arg.parse::<i32>().is_ok()) {
                let new_call = format!("{}_{}()", function_name.as_str(), args.join("_"));
                let reparsed = VerusParser::parse(Rule::expr, new_call.as_str());
                // println!("handeld out = {:?}", reparsed.clone().unwrap().as_str());
                VerusVisitor::visit_all(datum, reparsed.unwrap(), handlers);
                for inner_nested_pair in nested_exprs_list {
                    // handle any nested exprs
                    VerusVisitor::visit(datum, inner_nested_pair, handlers);
                }
                handled = true;
            }
        }
        if !handled {
            VerusVisitor::default_visit(datum, pair, handlers);
        }
    }

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.variable_stack.push(vec![]);
        datum.program_mut().push_str("verus!{\n");
        VerusVisitor::visit_all(datum, pair.clone().into_inner(), handlers);
        println!(
            "Functions found (fn_map keys): {:?}",
            datum.fn_map.keys().collect::<Vec<&String>>()
        );
        println!("Function Calls (fn_calls): {:?}", datum.fn_calls);

        for call in datum.fn_calls.keys() {
            if datum.fn_map.contains_key(call) {
                if let Some(arg_sets) = datum.fn_calls.get(call) {
                    for args in arg_sets {
                        if args.iter().all(|arg| arg.parse::<i32>().is_ok()) {
                            //todo:
                            //parse the string, create a visitor that runs it and print the progn output
                            // lets assume the pattern of name_args for a fn call is unique to this..
                            let inlined_fn_name = format!("{}_{}", call, args.clone().join("_"));

                            {
                                let mut handled_fns = HANDLED_INLINE_FNS.lock().unwrap();
                                if !handled_fns.contains(&inlined_fn_name.clone()){
                                    handled_fns.insert(inlined_fn_name.clone());
                                    // println!("fncs handled = {:?}",handled_fns);
                                    let reparsed =
                                    VerusParser::parse(Rule::r#fn, datum.fn_map[call].as_str());
                                    let mut d = InlinerDatum {
                                        program: "".to_string(),
                                        inlined_args: args.clone(),
                                        original_args: HashMap::new(),
                                    };
                                    InlineSingleFunctionCallVisitor::inline_func(&mut d, reparsed.unwrap());
                                    datum.program += format!("\n{}", d.program).as_str();
                                    datum.fn_map.insert(inlined_fn_name, d.program);
                                }
                            }
                        }
                    }
                }
            }
        }
        datum.program_mut().push_str("}");
    }

    fn visit_fn_block_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("\n {");
        datum.variable_stack.push(vec![]);
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("\n } \n");
        for entry in datum.variable_stack.last_mut().unwrap() {
            datum.variable_map.remove(entry);
        }
        datum.variable_stack.pop();
    }
}

#[derive(Clone, Debug)]
pub struct InlinerDatum {
    pub program: String,
    pub inlined_args: Vec<String>,
    pub original_args: HashMap<String,String>,
    // pub original_args: Vec<String>,
}

impl HasProgram for InlinerDatum {
    fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }
}

pub struct InlineSingleFunctionCallVisitor {}

impl InlineSingleFunctionCallVisitor {
    fn create_combined_handler_map() -> HandlerMap<InlinerDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", InlineSingleFunctionCallVisitor::visit_function);
        handlers.insert(
            "identifier",
            InlineSingleFunctionCallVisitor::visit_identifier,
        );
        handlers.insert("arg_list", InlineSingleFunctionCallVisitor::visit_arg_list);
        handlers
    }

    fn visit_function(
        datum: &mut InlinerDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<InlinerDatum>,
    ) {
        let mut param_name: Option<String> = None;
        let mut param_type: Option<String> = None;
    
        for fn_comp in pair.clone().into_inner() {
            match fn_comp.as_rule() {
                Rule::name => {
                    let new_name = format!("{}_{}", fn_comp.as_str(), datum.inlined_args.join("_"));
                    let new_name_rule = VerusParser::parse(Rule::name, new_name.as_str())
                        .map_err(ParseAndFormatError::from)
                        .expect("")
                        .next()
                        .unwrap();
                    VerusVisitor::visit(datum, new_name_rule, handlers)
                }
                Rule::param_list => {
                    datum.program += "(";
                    for param in fn_comp.clone().into_inner() {
                        for pc in param.clone().into_inner() {
                            if pc.as_rule() == Rule::r#type {
                                param_type = Some(pc.as_str().to_string());
                            }
                            if pc.as_rule() == Rule::pat_no_top_alt {
                                param_name = Some(pc.as_str().to_string());
                            }
                        }
    
                        // Insert param_name into the map with param_type or an empty string if param_type is None
                        if let Some(param_name) = param_name.take() {
                            let param_type_value = param_type.take().unwrap_or_else(String::new);
                            datum.original_args.insert(param_name, param_type_value);
                        }
                    }
                    datum.program += ") ";
                }
                _ => VerusVisitor::visit(datum, fn_comp, handlers),
            }
        }
    }
    
    fn visit_arg_list(
        datum: &mut InlinerDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<InlinerDatum>,
    ) {
        let mut arg_list_replacement = String::from("("); // Start the argument list
    
        let mut first = true; // Track whether it's the first argument for comma placement
    
        for inner_pair in pair.clone().into_inner() { // Iterate over comma-delimited arguments
    
            for inner_inner_pair in inner_pair.clone().into_inner() { // Iterate over inner expression components
                // println!(
                //     "inner----- exprs {:?} , {:?}",
                //     inner_inner_pair.as_rule(),
                //     inner_inner_pair.as_str()
                // );
    
                let param_arg_exprs = inner_inner_pair.as_str();
    
                if let Some(index) = datum.original_args.keys().position(|key| key == param_arg_exprs) {
                    let inlined_value = datum
                        .inlined_args
                        .get(index)
                        .expect("Index out of bounds")
                        .clone();
    
                    // Append a comma only if it's not the first argument
                    if !first {
                        arg_list_replacement.push_str(", ");
                    }
                    arg_list_replacement.push_str(&inlined_value);
                } else {
                    // If no replacement, use the original value
                    if !first {
                        arg_list_replacement.push_str(", ");
                    }
                    arg_list_replacement.push_str(param_arg_exprs);
                }
                first = false; // Subsequent arguments will need commas
            }
        }
    
        arg_list_replacement.push(')'); // Close the argument list
        let new_val = VerusParser::parse(Rule::arg_list, arg_list_replacement.as_str())
            .map_err(ParseAndFormatError::from)
            .expect("Parsing inlined argument failed")
            .next()
            .unwrap();
        datum.program_mut().push_str(new_val.as_str());
    }
    




    pub fn visit_identifier(
        datum: &mut InlinerDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<InlinerDatum>,
    ) {
        let identifier = pair.as_str();
        // Find the corresponding index in original_args
        if let Some(index) = datum.original_args.keys().position(|key| key == identifier) {
            // Retrieve the inlined value from inlined_args using the index
            let inlined_value = datum.inlined_args.get(index).expect("Index out of bounds").clone();
            // Parse the inlined value
            let typed_inlined_value = format!(
                "{}{}",
                inlined_value,
                datum.original_args.get(identifier).map_or("", |v| v.as_str())
            );            
            
            // let new_val = VerusParser::parse(Rule::int_number, &inlined_value)
            let new_val = VerusParser::parse(Rule::int_number, typed_inlined_value.as_str())
                .map_err(ParseAndFormatError::from)
                .expect("Parsing inlined argument failed")
                .next()
                .unwrap();
            // Visit the parsed value
            VerusVisitor::visit(datum, new_val, handlers);
        } else {
            VerusVisitor::default_visit(datum, pair, handlers);
        }
    }

    pub fn inline_func(datum: &mut InlinerDatum, pairs: Pairs<Rule>) {
        let handler_map: HandlerMap<InlinerDatum> =
            InlineSingleFunctionCallVisitor::create_combined_handler_map();

        VerusVisitor::visit_all(
            datum,
            pairs,
            &handler_map as &dyn HandlerInterface<InlinerDatum>,
        );
    }
}

use crate::ParseAndFormatError;
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::HashMap;
pub trait HasProgram {
    fn program(&self) -> &String;
    fn program_mut(&mut self) -> &mut String;
}

pub trait HandlerInterface<T: HasProgram> {
    fn get_handler(&self, rule: &str) -> Option<fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)>;
}

pub struct HandlerMap<T: HasProgram> {
    handlers: HashMap<&'static str, fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)>,
}

impl<T: HasProgram> HandlerMap<T> {
    // Initialize a new handler map
    pub fn new() -> Self {
        let mut handlers: HashMap<&'static str, fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)> =
            HashMap::new();

        // Insert handlers into the map
        handlers.insert("verus_macro_use", VerusVisitor::visit_verus_macro_use);
        handlers.insert("param_list", VerusVisitor::visit_param_list);
        handlers.insert("fn_block_expr", VerusVisitor::visit_fn_block_expr);
        handlers.insert("stmt_list", VerusVisitor::visit_stmt_list);
        handlers.insert("closure_param_list", VerusVisitor::visit_closure_param_list);
        handlers.insert("generic_param_list", VerusVisitor::visit_generic_param_list); // new
        handlers.insert(
            "comma_delimited_exprs",
            VerusVisitor::visit_comma_delimited_exprs,
        );
        handlers.insert(
            "comma_delimited_exprs_for_verus_clauses",
            VerusVisitor::visit_comma_delimited_exprs_for_verus_clauses,
        );
        handlers.insert("paren_expr_inner", VerusVisitor::visit_paren_expr_inner);
        handlers.insert("arg_list", VerusVisitor::visit_arg_list);
        handlers.insert("COMMENT", VerusVisitor::visit_comment);
        handlers.insert("item_list", VerusVisitor::visit_item_list);
        handlers.insert("record_field_list", VerusVisitor::visit_record_field_list); 
        handlers.insert("assoc_item_list", VerusVisitor::visit_assoc_item_list); 
        handlers.insert("match_arm_list", VerusVisitor::visit_match_arm_list);
        handlers.insert("tuple_struct_pat_inner", VerusVisitor::visit_tuple_struct_pat_inner);
        handlers.insert("tuple_pat", VerusVisitor::visit_tuple_struct_pat);
        handlers.insert("match_arm_lhs", VerusVisitor::visit_match_arm_lhs);
        handlers.insert("record_expr_field_list", VerusVisitor::visit_record_expr_field_list);
        handlers.insert("trigger_str", VerusVisitor::visit_trigger_str);
        handlers.insert("generic_args", VerusVisitor::visit_generic_args);
        handlers.insert("ref_type", VerusVisitor::visit_ref_type);
        handlers.insert("use_tree_list", VerusVisitor::visit_use_tree_list);
        handlers.insert("attr_core", VerusVisitor::visit_attr_core);
        handlers.insert("variant_list", VerusVisitor::visit_variant_list);
        handlers.insert("field_list", VerusVisitor::visit_field_list);
        handlers.insert("condensable_record_field_list", VerusVisitor::visit_condensable_record_field_list);
        handlers.insert("record_pat_field_list", VerusVisitor::visit_record_pat_field_list);
        Self { handlers }
    }

    pub fn insert(
        &mut self,
        key: &'static str,
        handler: fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>),
    ) {
        self.handlers.insert(key, handler);
    }
}

impl<T: HasProgram> HandlerInterface<T> for HandlerMap<T> {
    fn get_handler(&self, rule: &str) -> Option<fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)> {
        self.handlers.get(rule).copied() // return the function pointer
    }
}

pub struct VerusVisitor;

impl VerusVisitor {
    fn visit<T: HasProgram>(datum: &mut T, pair: Pair<Rule>, handlers: &dyn HandlerInterface<T>) {
        // println!("VISITING {:?} {:?}", pair.as_rule(), pair.as_str());
        let rule_name = format!("{:?}", pair.as_rule());
        if let Some(handler) = handlers.get_handler(&rule_name) {
            handler(datum, pair, handlers);
        } else {
            VerusVisitor::default_visit(datum, pair, handlers);
        }
    }

    fn default_visit<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        // println!("VISITING {:?} : {}", pair.as_rule(), pair.as_str());
        let inner_pairs = pair.clone().into_inner();
        if inner_pairs.clone().count() == 0 {
            datum.program_mut().push_str(&format!("{} ", pair.as_str()));
        } else {
            VerusVisitor::visit_all(datum, inner_pairs, handlers);
        }
    }
    fn visit_attr_core<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str(&format!("{} ", pair.as_str()));
    }


    fn visit_record_pat_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{ ");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str(" }");
    }
    
    fn visit_condensable_record_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        let mut inner_pairs = pair.into_inner().peekable(); // Make the iterator peekable
    
        while let Some(inner_pair) = inner_pairs.next() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
    
            // Add "," only if there's another element after the current one
            if inner_pairs.peek().is_some() {
                datum.program_mut().push_str(", ");
            }
        }
    }

    fn visit_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        let first_char = pair.as_str().chars().next();

        if let Some('{') = first_char {
            // Handle curly brace case
            datum.program_mut().push_str("{");
            for inner_pair in pair.into_inner() {
                VerusVisitor::visit(datum, inner_pair, handlers);
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str("}");
        } else if let Some('(') = first_char {
            // Handle parenthesis 
            datum.program_mut().push_str("(");
            for inner_pair in pair.into_inner() {
                VerusVisitor::visit(datum, inner_pair, handlers);
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(")");
        }
        
    }


    fn visit_variant_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }

    fn visit_item_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_record_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            datum.program_mut().push_str(inner_pair.as_str());
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }

    fn visit_assoc_item_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_match_arm_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_tuple_struct_pat_inner<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str(")");
    }

    fn visit_tuple_struct_pat<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }        datum.program_mut().push_str(")");
    }

    fn visit_match_arm_lhs<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("=>");
    }

    fn visit_record_expr_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }
    
    fn visit_trigger_str<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("[trigger]");
    }

    fn visit_generic_args<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
    }

    fn visit_ref_type<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("&");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_use_tree_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }        
        datum.program_mut().push_str("}");

    }

    

    fn visit_stmt_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_verus_macro_use<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("verus!{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_generic_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("<");
        for inner_pair in pair.into_inner() {
            VerusVisitor::default_visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str(">");
    }


    fn visit_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('(');
        let mut first = true;
        for inner_pair in pair.into_inner() {
            if !first {
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(inner_pair.as_str());
            first = false;
        }
        datum.program_mut().push(')');
    }

    fn visit_fn_block_expr<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("\n {");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("\n } \n");
    }

    fn visit_paren_expr_inner<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str(")");
    }

    fn visit_closure_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('|');
        let mut first = true;
        for inner_pair in pair.into_inner() {
            if !first {
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(inner_pair.as_str());
            first = false;
        }
        datum.program_mut().push('|');
    }

    fn visit_comma_delimited_exprs<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
    }

    fn visit_comma_delimited_exprs_for_verus_clauses<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::COMMENT => {
                    // dont add "extra" comma if element is comment
                }
                _ => {
                    VerusVisitor::visit(datum, inner_pair, handlers);
                    datum.program_mut().push_str(", \n");
                }
            }           
        }
    }

    fn visit_arg_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('(');
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push(')');
    }

    fn visit_comment<T: HasProgram>(
        _datum: &mut T,
        _pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        // Do nothing for comments
        //[TODO] : Comments in the middle of comma separated list:
        //i.e. this example causes an error
        // ensures f.len() == n,
        // f[0] == 0,  // comments in the middle of comma list still not covered
        // f[n-1] != 0,
    }

    pub fn visit_all<T: HasProgram>(
        datum: &mut T,
        pairs: Pairs<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for pair in pairs {
            VerusVisitor::visit(datum, pair, handlers);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CoreDatum {
    pub program: String,
    pub fn_map: HashMap<String, String>, // Assume names are unique for now
    pub fn_calls: HashMap<String, Vec<Vec<String>>>,
    pub target_name: String, // Add target_name
    pub finite_bound: usize,
    pub variable_map: HashMap<String, String>,
    pub variable_stack: Vec<Vec<String>>,
}

// Implement HasProgram for CoreDatum
impl HasProgram for CoreDatum {
    fn program(&self) -> &String {
        &self.program
    }

    fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }
}
impl CoreDatum {
    pub fn get_target_name(&self) -> &String {
        &self.target_name
    }
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

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    let mut nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        function_name = Some(function_pair.as_str().to_string());
                    }
                }
                Rule::arg_list => {
                    let args: String = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect();
                    arguments = Some(args.clone());
                }
                _ => {}
            }
        }

        let mut handled = false;
        if let (Some(function_name), Some(arguments)) = (function_name, arguments) {
            println!(
                "Function called: {} with args: {:?}",
                function_name, arguments
            );
            let args: Vec<String> = arguments
                .as_str()
                .split(',')
                .map(|s| {
                    let trimmed = s.trim().to_string();
                    datum.variable_map.get(&trimmed).cloned().unwrap_or(trimmed)
                })
                .collect();

            datum
                .fn_calls
                .entry(function_name.clone())
                .or_insert_with(Vec::new)
                .push(args.clone());

            if args.iter().all(|arg| arg.parse::<i32>().is_ok()) {
                let new_call = format!("{}_{}()", function_name.as_str(), args.join("_"));
                let reparsed = VerusParser::parse(Rule::expr, new_call.as_str());
                VerusVisitor::visit_all(datum, reparsed.unwrap(), handlers);
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
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
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

                            let reparsed =
                                VerusParser::parse(Rule::r#fn, datum.fn_map[call].as_str());
                            let mut d = InlinerDatum {
                                program: "".to_string(),
                                inlined_args: args.clone(),
                                original_args: vec![],
                            };
                            InlineSingleFunctionCallVisitor::inline_func(&mut d, reparsed.unwrap());
                            datum.program += format!("\n{}", d.program).as_str();
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
    pub original_args: Vec<String>,
}

impl HasProgram for InlinerDatum {
    fn program(&self) -> &String {
        &self.program
    }

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
        handlers
    }

    fn visit_function(
        datum: &mut InlinerDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<InlinerDatum>,
    ) {
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
                            if pc.as_rule() == Rule::pat_no_top_alt {
                                datum.original_args.push(pc.as_str().to_string());
                            }
                        }
                    }
                    datum.program += ") ";
                }
                _ => VerusVisitor::visit(datum, fn_comp, handlers),
            }
        }
    }

    pub fn visit_identifier(
        datum: &mut InlinerDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<InlinerDatum>,
    ) {
        let identifier = pair.as_str();
        if let Some(index) = datum.original_args.iter().position(|arg| arg == identifier) {
            let inlined_value = datum.inlined_args.get(index).unwrap().clone();
            let new_val = VerusParser::parse(Rule::int_number, inlined_value.as_str())
                .map_err(ParseAndFormatError::from)
                .expect("Parsing inlined argument failed")
                .next()
                .unwrap();
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

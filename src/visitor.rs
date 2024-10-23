use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub trait HasProgram {
    fn program(&self) -> &String;
    fn program_mut(&mut self) -> &mut String;
}

trait HandlerInterface<T: HasProgram> {
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
        handlers.insert("closure_param_list", VerusVisitor::visit_closure_param_list);
        handlers.insert(
            "comma_delimited_exprs",
            VerusVisitor::visit_comma_delimited_exprs,
        );
        handlers.insert("arg_list", VerusVisitor::visit_arg_list);
        handlers.insert("COMMENT", VerusVisitor::visit_comment);

        Self { handlers }
    }

    fn insert(
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
        //println!("VISITING {:?} {:?}", pair.as_rule(), pair.as_str());
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
        let inner_pairs = pair.clone().into_inner();
        if inner_pairs.clone().count() == 0 {
            datum.program_mut().push_str(&format!("{} ", pair.as_str()));
        } else {
            VerusVisitor::visit_all(datum, inner_pairs, handlers);
        }
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
    }

    fn visit_all<T: HasProgram>(
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
    pub fn_map: HashMap<String, String>, //assume names are unique for now
    pub fn_calls: HashMap<String, Vec<Vec<String>>>,
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
pub struct CoreVerusVisitor {}

impl CoreVerusVisitor {
    fn create_combined_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();

        handlers.insert("fn", CoreVerusVisitor::visit_function);
        handlers.insert("expr", CoreVerusVisitor::visit_expr);
        handlers.insert("verus_macro_use", CoreVerusVisitor::visit_verus_macro_use);
        handlers
    }

    pub fn visit_all(datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = CoreVerusVisitor::create_combined_handler_map();
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

    fn visit_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut inner_pairs = pair.clone().into_inner();

        let mut function_name: Option<String> = None;
        let mut arguments: Option<Vec<String>> = None;

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
                    let args: Vec<String> = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect();
                    arguments = Some(args.clone());
                }
                _ => {}
            }
        }

        if let (Some(function_name), Some(arguments)) = (function_name, arguments) {
            println!(
                "Function called: {} with args: {:?}",
                function_name, arguments
            );
            datum
                .fn_calls
                .entry(function_name)
                .or_insert_with(Vec::new)
                .push(arguments);
        }

        VerusVisitor::default_visit(datum, pair, handlers);
    }

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        VerusVisitor::visit_verus_macro_use(datum, pair, handlers);
        println!(
            "Functions found (fn_map keys): {:?}",
            datum.fn_map.keys().collect::<Vec<&String>>()
        );
        println!("Function Calls (fn_calls): {:?}", datum.fn_calls);
    }
}
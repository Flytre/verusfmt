use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use std::sync::{Mutex};
use lazy_static::lazy_static;

// Define a global mutable variable to store the function name
lazy_static! {
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
}

// Define a new struct for your custom visitor
pub struct RecursionVisitor {
    target_name: String, // Store target_name within RecursionVisitor
}

impl RecursionVisitor {
    pub fn new(target_name: String) -> Self {
        RecursionVisitor { target_name } // Return an instance of RecursionVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("expr", RecursionVisitor::visit_expr);
        handlers.insert("fn", RecursionVisitor::visit_function);
        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }


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
        {
            let mut parent_name = PARENT_FUNCTION_NAME.lock().unwrap();
            *parent_name = name.to_string();
        }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

   

    fn visit_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let current_parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();
        println!("Parent function name is: {}", current_parent_name);
        // if fn call expr is the same as parent expr

        for inner_pair in pair.clone().into_inner() {
            println!("inner_pair =   {:?} :: {:?}", inner_pair.as_rule(),inner_pair.as_str());

        }
    }

}

use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;

// Define a new struct for your custom visitor
pub struct LoopVisitor {
    target_name: String, // Store target_name within LoopVisitor
}

impl LoopVisitor {
    pub fn new(target_name: String) -> Self {
        LoopVisitor { target_name } // Return an instance of LoopVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("identifier", LoopVisitor::visit_identifier);
        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }

    fn visit_identifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair.as_str();
        if name == "indexUpTo" {
            datum.program_mut().push_str(&format!("new_{} ", name));
        } else {
            datum.program_mut().push_str(&format!("{} ", name));
        }
    }


}

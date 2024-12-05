use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs

// Define a new struct for your custom visitor
pub struct SimpleVisitor {
    _target_name: String, // Store target_name within SimpleVisitor
}

impl SimpleVisitor {
    pub fn new(target_name: String) -> Self {
        SimpleVisitor { _target_name: target_name } // Return an instance of SimpleVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("custom_rule", SimpleVisitor::visit_custom_rule);
        handlers.insert("fn", SimpleVisitor::visit_function);
        handlers.insert("identifier", SimpleVisitor::visit_identifier);
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

    fn visit_identifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair.as_str();
        if name == datum.target_name {
            datum.program_mut().push_str(&format!("new_{} ", name));
        } else {
            datum.program_mut().push_str(&format!("{} ", name));
        }
    }

    /// Handler for the "fn" rule. This is a Core-specific handler.
    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_custom_rule(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("/* Custom Rule Start */");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("/* Custom Rule End */");
    }
}

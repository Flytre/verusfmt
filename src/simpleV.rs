use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;

// Define a new struct for your custom visitor

pub struct SimpleVisitor {
    target_name: String,
}

impl SimpleVisitor {

    pub fn new(target_name: String) -> Self {
        SimpleVisitor {
            target_name,
        }
    }

    
    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();

        //example rule
        handlers.insert("custom_rule", SimpleVisitor::visit_custom_rule);

        handlers.insert("fn", SimpleVisitor::visit_function);
        handlers.insert("identifier", SimpleVisitor::visit_identifier);

        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }


    fn visit_identifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair.as_str();
        if(name == "is_prime"){ //hard-coded for now
            datum.program_mut().push_str(&format!("new_{} ", pair.as_str()));
        }else{
            datum.program_mut().push_str(&format!("{} ", pair.as_str()));
        }
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
        
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }



    fn visit_custom_rule(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // Custom handling logic here
        datum.program_mut().push_str("/* Custom Rule Start */");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("/* Custom Rule End */");
    }


}
use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PARENT_FUNCTION_NAME_MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_PARAM_LIST_MAP: Mutex<HashMap<String, Vec<String>>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_QUALIFIER_MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
}

// Define a new struct for your custom visitor
pub struct PreProcessVisitor {}

impl PreProcessVisitor {
    pub fn new() -> Self {
        PreProcessVisitor {} // Return an instance of PreProcessVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", PreProcessVisitor::visit_function);
        handlers.insert(
            "verus_macro_use",
            PreProcessVisitor::visit_verus_macro_use,
        );

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
        let mode = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::fn_mode);
        let param_list = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::param_list);
        let fn_qualifier = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::fn_qualifier);
        println!("Stored the function body for {}", name);

        datum
            .fn_map
            .insert(name.to_string(), pair.as_str().to_string());
        {
            let mut parent_names = PARENT_FUNCTION_NAME_MAP.lock().unwrap();
            if let Some(mode) = mode {
                parent_names.insert(name.to_string(), mode.as_str().to_string());
            } else {
                parent_names.insert(name.to_string(), "fn".to_string());
            }
        }
        {
            let mut parent_param_list = PARENT_FUNCTION_PARAM_LIST_MAP.lock().unwrap();
            if let Some(param_list) = param_list {
                // println!("Parameters for {}:", name);

                let mut params_vec = Vec::new(); // Initialize a vector to hold parameter strings

                for param in param_list.clone().into_inner() {
                    // println!("Param = {:?} {:?}", param.as_str(), param.as_rule());

                    let inner_param = param.clone().into_inner();
                    for innerp in inner_param {
                        if innerp.as_rule() == Rule::pat_no_top_alt {
                            // println!("Inner Param = {:?} {:?}", innerp.as_str(), innerp.as_rule());
                            params_vec.push(innerp.as_str().to_string());
                        }
                    }
                }

                parent_param_list.insert(name.to_string(), params_vec);
            } else {
                parent_param_list.insert(name.to_string(), Vec::new());
            }
        }
        {
            let mut parent_qualifiers = PARENT_FUNCTION_QUALIFIER_MAP.lock().unwrap();
            if let Some(fn_qualifier) = fn_qualifier {
                parent_qualifiers.insert(name.to_string(), fn_qualifier.as_str().to_string());
            } else {
                parent_qualifiers.insert(name.to_string(), "()".to_string());
            }
        }
        {
            let mut parent_name = PARENT_FUNCTION_NAME.lock().unwrap();
            *parent_name = name.to_string();
        }

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

  

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("verus!{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}");
    }
}
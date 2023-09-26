#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Router, Route};
use fermi::use_init_atom_root;

use crate::page;





pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    cx.render(rsx! {

        Router {
            Route { to: page::ACCOUNT_REGISTER , page::Register {} },
            
        }
    
        
        
       
    })
}

mod utils;

use scheme_rs::environment::Signal;
use scheme_rs::lisp_val::LispVal;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use scheme_rs::Thingus;

// TODO: Any way to avoid re-wrapping this?
#[wasm_bindgen]
pub struct Thing(Thingus);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn setTimeout(f: &js_sys::Function, time: i32);
}

fn wrap_signal(signal: js_sys::Function) -> Signal {
    Box::new(move |vals| {
        // TODO! HACK! This works around a refcell borrow issue :S
        // E.g. The port is borrowed in order to write to it, the port is
        // then signalled (i.e. the port callback is called). If an operation
        // is then performed on the port that also requires borrowing the
        // port (e.g. `read_port`), the original borrow is still in effect,
        // and we get a panic. The library
        // should handle this, the caller shouldn't have to know about this,
        // but this might need some further thinking...
        let val = vals
            .iter()
            .filter(|val| **val != LispVal::Void)
            .map(|val| format!("{}", val))
            .collect::<Vec<String>>()
            .join("\n");
        setTimeout(&signal.bind1(&JsValue::NULL, &JsValue::from_str(&val)), 0)
    })
}

#[wasm_bindgen]
impl Thing {
    pub fn new(js_signal: &js_sys::Function) -> Self {
        set_panic_hook();
        let owned_signal = js_signal.to_owned();
        let wrapped_signal = wrap_signal(owned_signal);
        Thing(Thingus::new(wrapped_signal))
    }
    pub fn eval(&self, input: String) -> String {
        self.0.eval(&input)
    }
    pub fn read_port(&self, port: String) -> String {
        match self.0.ports.get(&port).take() {
            None => "Port not found".to_string(),
            Some(port) => {
                let mut port = port.borrow_mut();
                let vals = port.flush();
                vals.iter()
                    .filter(|val| **val != LispVal::Void)
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::*;

//     fn noop() {}

//     #[wasm_bindgen_test]
//     fn stuff_and_junk() {
//         let t = Thing::new(&js_sys::Function::new_no_args("console.log()"));
//         t.eval("(write \"ddd\")".to_string());
//         assert_eq!(t.read_port("default".to_string()), "dddx".to_string())
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn noop(_port: &mut Vec<LispVal>) {}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
    set_panic_hook(); // TODO: <- kinda weird putting this here?
    let t = Thingus::new(Box::new(noop));
    t.eval(input)
}

#[wasm_bindgen]
pub fn read_eval_print(input: &str) -> String {
    // TODO
    set_panic_hook(); // TODO: <- kinda weird putting this here?
    let t = Thingus::new(Box::new(noop));
    t.eval(input)
}

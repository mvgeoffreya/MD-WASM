mod parser;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[derive(Debug)]
    type HTMLDocument;
    #[derive(Debug)]
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn add_value(this: &Element, html: &str);

    #[wasm_bindgen (structural , method , setter , js_class = "Element" , js_name = className)]
    fn class_name(this: &Element, value: &str);

}

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let result = parser::parse(input.to_string());
    result
}
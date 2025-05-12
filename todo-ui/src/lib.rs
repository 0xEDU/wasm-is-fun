use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, console};

#[wasm_bindgen]
pub fn add_todo(todo: String) {
    let document = web_sys::window().unwrap().document().unwrap();
    console::log_1(&JsValue::from_str(&format!("Todo added: {}", todo)));
    let list_item = document.create_element("li").unwrap();
    let value_div = document.create_element("div").unwrap();
    value_div.set_inner_html(&todo);
    list_item.append_child(&value_div).unwrap();
    let list = document
        .get_element_by_id("todo-list")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    list.append_child(&list_item).unwrap();

    let remove_button = document.create_element("button").unwrap();
    remove_button.set_inner_html("Remove");
    remove_button
        .set_attribute("onclick", "document.wasm.remove_todo(event)")
        .unwrap();
    list_item.append_child(&remove_button).unwrap();

    let edit_button = document.create_element("button").unwrap();
    edit_button.set_inner_html("Edit");
    edit_button
        .set_attribute("onclick", "document.wasm.edit_todo(event)")
        .unwrap();
    list_item.append_child(&edit_button).unwrap();
}

#[wasm_bindgen]
pub fn remove_todo(e: web_sys::MouseEvent) {
    let target = e.target().unwrap();
    let remove_button = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let list = document
        .get_element_by_id("todo-list")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let list_item = remove_button.parent_node().unwrap();
    list.remove_child(&list_item).unwrap();
    list.remove_child(&remove_button).unwrap();
}

#[wasm_bindgen]
pub fn edit_todo(e: web_sys::MouseEvent) {
    let target = e.target().unwrap();
    let edit_button = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let list_item = edit_button.parent_node().unwrap();
    let _todo_text = list_item.text_content().unwrap();
    let input = document.create_element("input").unwrap();
    input.set_attribute("value", "banana").unwrap();
    list_item.replace_child(&input, &edit_button).unwrap();

    let save_button = document.create_element("button").unwrap();
    save_button.set_inner_html("Save");
    save_button
        .set_attribute("onclick", "document.wasm.save_todo(event)")
        .unwrap();
    list_item.append_child(&save_button).unwrap();
}

#[wasm_bindgen]
pub fn save_todo(e: web_sys::MouseEvent) {
    let target = e.target().unwrap();
    let save_button = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let list_item = save_button.parent_node().unwrap();
    let input = list_item
        .last_child()
        .unwrap()
        .previous_sibling()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let value_div = list_item
        .first_child()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    value_div.set_inner_html(&input.value());
    list_item.remove_child(&input).unwrap();
    list_item.remove_child(&save_button).unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let edit_button = document.create_element("button").unwrap();
    edit_button.set_inner_html("Edit");
    edit_button
        .set_attribute("onclick", "document.wasm.edit_todo(event)")
        .unwrap();
    list_item.append_child(&edit_button).unwrap();
}

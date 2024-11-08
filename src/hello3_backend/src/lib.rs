use ic_cdk::storage;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;


thread_local! {
    static SUBMITTED_NAMES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[update]
fn greet(name: String) -> String {
    // 将 name 添加到 SUBMITTED_NAMES 中
    SUBMITTED_NAMES.with(|names| names.borrow_mut().push(name.clone()));
    format!("Hello, {}!", name)
}

#[query]
fn get_submitted_names() -> Vec<String> {
    // 返回所有提交过的 name 参数
    SUBMITTED_NAMES.with(|names| names.borrow().clone())
}

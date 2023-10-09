#![allow(dead_code)]

extern crate web_sys;
extern crate js_sys;

use web_sys::console::{
  log_1,
  log as web_sys_log,
  info_1,
  info as web_sys_info,
  warn_1,
  warn as web_sys_warn
};
use wasm_bindgen::JsValue;

pub fn log(msg: impl Into<String>) {
  /* Outputs a message to the client's Web console.
   * 
   * # Arguments
   * 
   * * `msg` - A message of which its type can be converted to a [String](std::string::String).
   */
  log_1(&JsValue::from_str(&msg.into()));
}

pub fn log_all(msgs: &[&str]) {
  /* Outputs multiple messages to the client's web console.
   * 
   * # Arguments
   * 
   * * `msgs` - A [Vector](std::vec) of any number of [str](str).
   */
  web_sys_log(&as_js_array(msgs));
}

pub fn info(msg: impl Into<String>) {
  /* Outputs an informational message to the Web console.
   * 
   * # Arguments
   * 
   * * `msg` - A message of which its type can be converted to a [String](std::string::String).
   */
  info_1(&JsValue::from_str(&msg.into()));
}

pub fn info_all(msgs: &[&str]) {
  /* Outputs multiple informational messages to the client's web console.
   * 
   * # Arguments
   * 
   * * `msgs` - A [Vector](std::vec) of any number of [str](str).
   */
  web_sys_info(&as_js_array(msgs));
}

pub fn warn(msg: impl Into<String>) {
  /* Outputs an informational message to the Web console.
   * 
   * # Arguments
   * 
   * * `msg` - A message of which its type can be converted to a [String](std::string::String).
   */
  warn_1(&JsValue::from_str(&msg.into()));
}

pub fn warn_all(msgs: &[&str]) {
  /* Outputs multiple messages to the client's web console.
   * 
   * # Arguments
   * 
   * * `msgs` - A [Vector](std::vec) of any number of [str](str).
   */
  web_sys_warn(&as_js_array(msgs));
}

fn as_js_array(v: &[&str]) -> js_sys::Array {
  v.iter()
      .map(|x| JsValue::from_str(x))
      .collect::<js_sys::Array>()
}
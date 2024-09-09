//Original code found in: https://github.com/mozilla/rust-android-gradle/blob/master/samples/rust/src/lib.rs
//Under Apache License v2
extern crate jni;

use std::os::raw::c_char;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_cesarvaliente_rustandroid_MainActivity_invokeCallbackFromAppInit(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    let s = String::from("Hello from Rust from JNI");
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");
    env.call_method(callback, "callbackFromInit", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_cesarvaliente_rustandroid_MainActivity_invokeCallbackViaButton(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    let s = String::from("You clicked, we are writing this from Rust, and now we invoke the callback that will show up the dialog :-)");
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");
    env.call_method(callback, "callbackFromButton", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
}

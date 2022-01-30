use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

use jni_fn::jni_fn;

#[jni_fn("fr.orion78.keepassTotp.androidPlugin.Native")]
pub fn hello_world(env: JNIEnv, _class: JClass, input: JString) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let output = env
        .new_string(format!("Hello from native rust, {}!", input))
        .expect("Couldn't create java string!");

    output.into_inner()
}

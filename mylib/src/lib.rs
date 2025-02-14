use jni::JNIEnv;

use jni::objects::{GlobalRef, JClass, JObject, JString};

use jni::objects::JByteArray;
use jni::sys::{jint, jlong};

use std::{sync::mpsc, thread, time::Duration};

#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_helloByte<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
    input: JByteArray<'local>,
) -> JByteArray<'local> {
    let _input = env.convert_byte_array(&input).unwrap();
    let buf = [1; 2000];
    let output = env.byte_array_from_slice(&buf).unwrap();
    output
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_factAndCallMeBack(
    mut env: JNIEnv,
    _class: JClass,
    n: jint,
    callback: JObject,
) {
    let i = n as i32;
    let res: jint = (2..i + 1).product();

    env.call_method(callback, "factCallback", "(I)V", &[res.into()])
        .unwrap();
}

struct Counter {
    count: i32,
    callback: GlobalRef,
}

impl Counter {
    pub fn new(callback: GlobalRef) -> Counter {
        Counter {
            count: 0,
            callback: callback,
        }
    }

    pub fn increment(&mut self, env: &mut JNIEnv) {
        self.count = self.count + 1;
        env.call_method(
            &self.callback,
            "counterCallback",
            "(I)V",
            &[self.count.into()],
        )
        .unwrap();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let counter = Counter::new(global_ref);

    Box::into_raw(Box::new(counter)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterIncrement(
    mut env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let counter = &mut *(counter_ptr as *mut Counter);

    counter.increment(&mut env);
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterDestroy(
    _env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(counter_ptr as *mut Counter);
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_asyncComputation(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let jvm = env.get_java_vm().unwrap();
    let callback = env.new_global_ref(callback).unwrap();
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        tx.send(()).unwrap();

        let mut env = jvm.attach_current_thread().unwrap();

        for i in 0..11 {
            let progress = (i * 10) as jint;
            env.call_method(&callback, "asyncCallback", "(I)V", &[progress.into()])
                .unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    rx.recv().unwrap();
}

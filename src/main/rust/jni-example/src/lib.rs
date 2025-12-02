use jni::JNIEnv;
use jni::objects::{JClass, JStaticMethodID, GlobalRef};

pub struct TestObj {
    call_target_class: GlobalRef,
    raise_exc_method: JStaticMethodID,
    log_method: JStaticMethodID,
}

impl TestObj {
    fn new(env: &mut JNIEnv) -> Self {
        let call_target_class = env
            .find_class("jni/rust/scenario/CallTarget")
            .expect("Failed to find CallTarget class");

        let raise_exc_method = env
            .get_static_method_id(&call_target_class, "raiseExc", "()V")
            .expect("Failed to find raiseExc method");

        let log_method = env
            .get_static_method_id(&call_target_class, "log", "()V")
            .expect("Failed to find log method");

        let call_target_class = env.new_global_ref(call_target_class)
            .expect("Failed to create global ref");

        Self {
            call_target_class,
            raise_exc_method,
            log_method,
        }
    }

    pub fn raise_exc(&self, env: &mut JNIEnv) -> jni::errors::Result<()> {
        unsafe {
            env.call_static_method_unchecked(
                &self.call_target_class,
                self.raise_exc_method,
                jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void),
                &[],
            )
        }?;

        Ok(())
    }

    pub fn log(&self, env: &mut JNIEnv) {
        let res = unsafe {
            env.call_static_method_unchecked(
                &self.call_target_class,
                self.log_method,
                jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void),
                &[],
            )
        };

        if let Err(err) = res {
            eprintln!("Could not call `CallTarget.log`: {err}");
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_jni_rust_scenario_TestObj_create(
    mut env: JNIEnv,
    _class: JClass,
) -> *mut TestObj {
    Box::into_raw(Box::new(TestObj::new(&mut env)))
}

#[no_mangle]
pub extern "system" fn Java_jni_rust_scenario_TestObj_invoke(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *const TestObj,
) {
    eprintln!("Java_jni_rust_scenario_TestObj_invoke :: (1)");
    let test_obj = unsafe { &*ptr };
    if test_obj.raise_exc(&mut env).is_err() {
        eprintln!("Java_jni_rust_scenario_TestObj_invoke :: (2)");
        test_obj.log(&mut env);
        eprintln!("Java_jni_rust_scenario_TestObj_invoke :: (3)");
    }
    eprintln!("Java_jni_rust_scenario_TestObj_invoke :: (4)");
}

#[no_mangle]
pub extern "system" fn Java_jni_rust_scenario_TestObj_destroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: *mut TestObj,
) {
    let test_obj = unsafe { Box::from_raw(ptr) };
    drop(test_obj);
}

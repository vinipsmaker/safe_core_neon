#[macro_use]
extern crate neon;
extern crate maidsafe_utilities;

use std::sync::mpsc;

use maidsafe_utilities::thread;
use neon::vm::{Call, JsResult, Module};
use neon::js::{JsFunction, JsString};

enum Action {
    CreateAccountAsync(String, String, JsFunction),
}

struct Singleton {
    sender: Option<mpsc::sync::Sender<Action>>,
    scope: Option<>,
}

impl Singleton {
    fn get() -> Singleton {
        #[derive(Clone)]
        struct Outer {
            inner: Arc<Mutex<Singleton>>,
        }

        static mut SINGLETON: *const Outer = 0 as *const Outer;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                let singleton = Outer {
                    sender: Arc::new(Mutex::new(Singleton { inner: None }))
                };

                SINGLETON = mem::transmute(Box::new(singleton));
            });

            (*SINGLETON).clone()
        }
    }
}

fn create_account_async(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let account_locator = try!(try!(call.arguments.require(scope, 0))
                               .check::<JsString>()).value();
    let account_password = try!(try!(call.arguments.require(scope, 1))
                                .check::<JsString>()).value();
    let callback =  = try!(try!(call.arguments.require(scope, 1))
                           .check::<JsFunction>());
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
    let (tx, rx) = mpsc::channel();

    {
        let singleton = Singleton::get();
        let mut guard = singleton.sender.lock();
        *guard = Some(tx);
    }

    thread::named("safe_core thread", move || {
        for e in rx {
            match e {
                Action::CreateAccountAsync(s) => (),
            }
        }
    }).detach();
    m.export("create_account_async", create_account_async)
});

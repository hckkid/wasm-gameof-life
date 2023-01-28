mod utils;

use wasm_bindgen::JsCast;
// use std::thread;
// use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::{console, Event, EventListener};
use web_sys::HtmlCanvasElement;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Counter {
    count: i64
}

#[wasm_bindgen]
impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    pub fn inc(&mut self) {
        self.count += 1;
    }

    pub fn log(&self) {
        use web_sys::console;
        console::log_1(&(self.count).into());
    }
}

#[wasm_bindgen]
pub struct Mycanvas {
    elm: HtmlCanvasElement,
    width: i32,
    height: i32
}

#[wasm_bindgen]
impl Mycanvas {
    pub fn new(elm: HtmlCanvasElement,width:i32, height:i32) -> Mycanvas {
        let mut obj = Mycanvas {
            elm,
            width,
            height
        };
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            console::log_1(&"Clicked".into());
        });
        obj.elm.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
        closure.forget();
        obj
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn resize(&mut self) {
        use web_sys::console;
        // thread::spawn(|| {
        //     for i in 1..10 {
        //         console::log_2(&"hi number from the spawned thread!".into(), &i.into());
        //         thread::sleep(Duration::from_millis(1));
        //     }
        // });
        //
        // for i in 1..5 {
        //     console::log_2(&"hi number from the main thread!".into(), &i.into());
        //     thread::sleep(Duration::from_millis(1));
        // }
        console::log_1(&"Will resize canvas".into());
        self.elm.set_height(self.height as u32);
        self.elm.set_width(self.width as u32);
    }

    // pub fn onmousedown(&mut self, _ev: Event) {
    //     console::log_1(&"Clicked".into());
    // }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-gameof-life!");
}

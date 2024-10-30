use wasm_bindgen::prelude::*;
#[wasm_bindgen(inline_js = "
    function wait(ms) {
       return new Promise((_, reject) => {
          setTimeout(() => reject(new Error('timeout succeeded')), ms);
       });
    }
    export fn js_timeout(ms, promise) {
        await Promise.race([wait(ms), promise()])
    }
    ")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_timeout(promise: js_sys::Promise) -> Result<JsValue, JsValue>;
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser );

    #[wasm_bindgen_test]
    fn it_works() {
        use super::stylesheet;

        stylesheet();
    }
}
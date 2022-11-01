#![allow(dead_code, type_alias_bounds)]

fn main() {
    println!("compiled successfully!")
}

#[rustfmt::skip]
mod imports {
    test_helpers::codegen_wasmer_export!(
        "*.wit"
 
        // TODO: implement async support
        "!async-functions.wit"

        // If you want to exclude a specific test you can include it here with
        // gitignore glob syntax:
        //
        // "!wasm.wit"
        // "!host.wit"
        //
        //
        // Similarly you can also just remove the `*.wit` glob and list tests
        // individually if you're debugging.
    );
}

mod exports {
    test_helpers::codegen_wasmer_import!(
        "*.wit"

        // TODO: implement async support
        "!async-functions.wit"

        // TODO: these use push/pull buffer which isn't implemented in the test
        // generator just yet
        "!wasi-next.wit"
        "!host.wit"
    );
}

mod custom_errors {
    wit_bindgen_host_wasmer_rust::export!({
        src["x"]: "
            foo: func()
            bar: func() -> expected<unit, u32>
            enum errno {
                bad1,
                bad2,
            }
            baz: func() -> expected<u32, errno>
        ",
        custom_error: true,
    });
}

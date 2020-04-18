pub fn html() -> String {
    format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
            {font_nunito}
            {rocksalt_style}
            <script>
                var Module = {{}};
                var __cargo_web = {{}};
                {rocksalt_wasm}
                Object.defineProperty( Module, 'canvas', {{
                    get: function() {{
                        if( __cargo_web.canvas ) {{
                            return __cargo_web.canvas;
                        }}

                        var canvas = document.createElement( 'canvas' );
                        document.querySelector( 'body' ).appendChild( canvas );
                        __cargo_web.canvas = canvas;

                        return canvas;
                    }}
                }});
            </script>
        </head>
        <body>
            {rocksalt_frontend}
        </body>
        </html>
		"#,
        rocksalt_wasm = inline_wasm(include_bytes!("../static/rocksalt-frontend.wasm").to_vec()),
        rocksalt_frontend = inline_script(include_str!("../static/rocksalt-frontend.js").to_string()),
        rocksalt_style = inline_style(include_str!("../static/rocksalt-style.css").to_string()),
        font_nunito = inline_style(include_str!("../static/font-nunito.css").to_string()),
    )
}

fn inline_style(style: String) -> String {
    format!(r#"<style type="text/css">{}</style>"#, style)
}

fn inline_wasm(wasm_file: Vec<u8>) -> String {
    format!(r#"var wasmCode = new Uint8Array({:?});"#, wasm_file)
}

fn inline_script(script: String) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, replace_yew_wasm_with_inline_wasm(script))
}

fn replace_yew_wasm_with_inline_wasm(file_contents: String) -> String {
    file_contents.replace(YEW_WASM_LOADER, INLINE_WASM_LOADER)
}

const YEW_WASM_LOADER: &'static str =
r#"if( typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string" ) {
            var fs = require( "fs" );
            var path = require( "path" );
            var wasm_path = path.join( __dirname, "rocksalt-frontend.wasm" );
            var buffer = fs.readFileSync( wasm_path );
            var mod = new WebAssembly.Module( buffer );
            var wasm_instance = new WebAssembly.Instance( mod, instance.imports );
            return instance.initialize( wasm_instance );
        } else {
            var file = fetch( "rocksalt-frontend.wasm", {credentials: "same-origin"} );

            var wasm_instance = ( typeof WebAssembly.instantiateStreaming === "function"
                ? WebAssembly.instantiateStreaming( file, instance.imports )
                    .then( function( result ) { return result.instance; } )

                : file
                    .then( function( response ) { return response.arrayBuffer(); } )
                    .then( function( bytes ) { return WebAssembly.compile( bytes ); } )
                    .then( function( mod ) { return WebAssembly.instantiate( mod, instance.imports ) } ) );

            return wasm_instance
                .then( function( wasm_instance ) {
                    var exports = instance.initialize( wasm_instance );
                    console.log( "Finished loading Rust wasm module 'rocksalt_frontend'" );
                    return exports;
                })
                .catch( function( error ) {
                    console.log( "Error loading Rust wasm module 'rocksalt_frontend':", error );
                    throw error;
                });
        }"#;

const INLINE_WASM_LOADER: &'static str =
r#"var wasm_instance = WebAssembly.compile(wasmCode)
.then( function(wasmModule) { return WebAssembly.instantiate(wasmModule, instance.imports); } );

return wasm_instance
.then( function( wasm_instance ) {
    return instance.initialize( wasm_instance );
})
.catch( function( error ) {
    throw error;
});"#;

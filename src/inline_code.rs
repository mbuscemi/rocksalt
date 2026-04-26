pub fn html() -> String {
    format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
            {font_nunito}
            {font_taviraj}
            {rocksalt_style}
            <script>var wasmCode = new Uint8Array({rocksalt_wasm:?});</script>
            {rocksalt_js}
            <script>wasm_bindgen.initSync({{ module: wasmCode }});</script>
        </head>
        <body>
        </body>
        </html>
        "#,
        font_nunito = inline_style(include_str!("../static/font-nunito.css")),
        font_taviraj = inline_style(include_str!("../static/font-taviraj.css")),
        rocksalt_style = inline_style(include_str!("../static/rocksalt-style.css")),
        rocksalt_wasm = include_bytes!("../frontend/pkg/rocksalt_frontend_bg.wasm"),
        rocksalt_js = inline_script(include_str!("../frontend/pkg/rocksalt_frontend.js")),
    )
}

fn inline_style(style: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, style)
}

fn inline_script(script: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, script)
}

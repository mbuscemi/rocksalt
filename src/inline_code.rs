use std::borrow::Cow;

const HTML: &[u8] = br#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="font-nunito.css" />
    <link rel="stylesheet" href="font-taviraj.css" />
    <link rel="stylesheet" href="rocksalt-style.css" />
    <script src="rocksalt_frontend.js"></script>
</head>
<body>
    <script>wasm_bindgen('rocksalt_frontend_bg.wasm');</script>
</body>
</html>"#;

pub fn serve(path: &str) -> Option<(Cow<'static, [u8]>, &'static str)> {
    match path {
        "/" | "/index.html" => Some((Cow::Borrowed(HTML), "text/html")),
        "/font-nunito.css" => Some((Cow::Borrowed(include_bytes!("../static/font-nunito.css")), "text/css")),
        "/font-taviraj.css" => Some((Cow::Borrowed(include_bytes!("../static/font-taviraj.css")), "text/css")),
        "/rocksalt-style.css" => Some((Cow::Borrowed(include_bytes!("../static/rocksalt-style.css")), "text/css")),
        "/rocksalt_frontend.js" => Some((Cow::Borrowed(include_bytes!("../frontend/pkg/rocksalt_frontend.js")), "text/javascript")),
        "/rocksalt_frontend_bg.wasm" => Some((Cow::Borrowed(include_bytes!("../frontend/pkg/rocksalt_frontend_bg.wasm")), "application/wasm")),
        _ => None,
    }
}

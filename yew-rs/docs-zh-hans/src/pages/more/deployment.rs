pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text(
            "当您准备将 Yew 应用程序部署到服务器时，您有多种部署方案可以选择。",
        )],
        p![
            code("trunk build --release"),
            text(" 会以发布模式构建您的应用程序。设置您的 HTTP 服务器，以便在访问您的站点时提供 "),
            code("index.html"),
            text("，并且对于静态路径（例如 "),
            code("index_<hash>.js"),
            text(" 和 "),
            code("index_bg_<hash>.wasm"),
            text("）的请求，应该从 trunk 生成的 dist 目录中提供相应的内容。"),
        ],
        admonition!(
            AdmonitionType::Important,
            Some("有关 `trunk serve --release`"),
            p![
                text("不要在生产环境中使用 "),
                code("trunk serve --release"),
                text(" 来提供您的应用程序。\n它只应该用于在开发过程中测试发布版本构建。"),
            ],
        ),
        h2![text("服务器配置")],
        h3![text("将 "), code("index.html"), text(" 作为回退提供")],
        p![
            text("如果应用程序使用了 "),
            link!("/zh-Hans/docs/concepts/router", text("Yew 路由")),
            text("，您必须配置服务器在请求不存在的文件时返回 "),
            code("index.html"),
            text("。"),
        ],
        p![
            text("具有 Yew 路由的应用程序被构建为 "),
            link!(
                "https://developer.mozilla.org/en-US/docs/Glossary/SPA",
                text("单页应用程序 (SPA)"),
            ),
            text("。当用户从正在运行的客户端导航到 URL 时，路由器会解释 URL 并路由到该页面。"),
        ],
        p![text(
            "但是在刷新页面或在地址栏中输入 URL \
             时，这些操作都是由浏览器本身处理的，而不是由正在运行的应用程序处理。\
             浏览器直接向服务器请求该 URL，绕过了路由器。错误配置的服务器会返回 404 - 未找到 \
             状态。",
        )],
        p![
            text("通过返回 "),
            code("index.html"),
            text("，应用程序会像通常一样加载，就好像请求是 "),
            code("/"),
            text("，直到路由器注意到路由是 "),
            code("/show/42"),
            text(" 并显示相应的内容。"),
        ],
        h3![text("为 Web Assembly 资源配置正确的 MIME 类型。")],
        p![
            text("WASM 文件必须使用 "),
            code("application/wasm"),
            text(" MIME 类型设置 "),
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type",
                text("Content-Type 头"),
            ),
            text("。"),
        ],
        p![text(
            "大多数服务器和托管服务默认已经这样做。如果您的服务器没有这样做，请查阅其文档。\
             在大多数 Web 浏览器中，错误的 MIME 类型会导致类似以下的错误：",
        )],
        code_block(
            "ignore",
            r#"`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:
 TypeError: WebAssembly: Response has unsupported MIME type 'text/plain' expected 'application/wasm'"#,
        ),
        h2![text("为相对路径构建")],
        p![
            text("默认情况下，trunk 会假定您的站点在 "),
            code("/"),
            text(" 处提供，并相应地构建站点。可以通过在 "),
            code("index.html"),
            text(" 文件中添加 "),
            code("<base data-trunk-public-url />"),
            text(" 来覆盖此行为。Trunk 会重写此标签以包含传递给 "),
            code("--public-url"),
            text(" 的值。Yew 路由会自动检测 "),
            code("<base />"),
            text(" 的存在并适当处理。"),
        ],
        h2![text("使用环境变量自定义行为")],
        p![
            text(
                "通常使用环境变量来自定义构建环境。由于应用程序在浏览器中运行，\
                 我们无法在运行时读取环境变量。 ",
            ),
            link!(
                "https://doc.rust-lang.org/std/macro.env.html",
                code("std::env!"),
            ),
            text(" 宏可以在编译时获取环境变量的值。"),
        ],
    ])
}

crate::doc_page!("部署", "/zh-Hans/docs/more/deployment", page_content());

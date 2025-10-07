use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

pub fn highlight(lang: &str, source: &str) -> String {
    let mut conf = match lang {
        "rust" => HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap(),
        "toml" => HighlightConfiguration::new(
            tree_sitter_toml::LANGUAGE.into(),
            "toml",
            tree_sitter_toml::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap(),
        "c" => HighlightConfiguration::new(
            tree_sitter_c::LANGUAGE.into(),
            "c",
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap(),

        "asciiart" => return source.to_owned(),

        "asm" => return source.to_owned(),
        _ => panic!("Unknown language: {}", lang),
    };

    let names: Vec<String> = conf.names().iter().map(|i| (*i).to_owned()).collect();
    conf.configure(&names);

    let mut highlighter = Highlighter::new();

    let highlights = highlighter
        .highlight(&conf, source.as_bytes(), None, |_| None)
        .unwrap();

    let classes: Vec<String> = names
        .iter()
        .map(|n| format!("class=\"hl-{}\"", n.replace('.', "-")))
        .collect();

    let mut html = HtmlRenderer::new();
    html.render(highlights, source.as_bytes(), &|hl, out| {
        let class = classes[hl.0].as_bytes();
        out.extend_from_slice(class);
    })
    .unwrap();

    String::from_utf8(html.html).unwrap()
}

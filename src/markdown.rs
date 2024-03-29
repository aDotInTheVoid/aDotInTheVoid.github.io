use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use pulldown_cmark::{html, CodeBlockKind};
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag};

use crate::config::Config;

pub fn render(conf: &Config, input: &str) -> String {
    let mut parser = Parser::new_ext(input, Options::all());
    let mut out = String::new();

    let mut footnotes = HashMap::new();

    let mut main_events = Vec::new();

    // let mut footnote_counter = 1;
    let mut numbers = HashMap::new();
    let mut names = Vec::new();

    let mut slugs = HashSet::new();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::FootnoteDefinition(name)) => {
                let old = footnotes.insert(name, collect_footnote(&mut parser));
                assert_eq!(old, None);
            }
            Event::Start(Tag::Heading(_, _, _)) => {
                main_events.push(event);

                let next_event = parser.next();
                let Some(Event::Text(t)) = next_event else {
                    panic!("got non-text heading {next_event:?}")
                };

                let mut slug = t.to_string().to_lowercase().replace(" ", "-");
                slug.retain(|x| x.is_alphanumeric() || x == '-');

                let link = format!(
                    "<a id=\"{slug}\" class=\"anchor\" href=\"#{slug}\" aria-hidden=\"true\"></a>"
                );
                main_events.push(Event::Html(CowStr::Boxed(link.clone().into_boxed_str())));

                assert_eq!(slugs.insert(slug), true, "Duplicate slug");

                main_events.push(Event::Text(t));
            }
            Event::Start(Tag::Image(kind, link, title)) => {
                let link = fix_link(link, conf);
                main_events.push(Event::Start(Tag::Image(kind, link, title)))
            }
            Event::Start(Tag::Link(kind, link, title)) => {
                let link = fix_link(link, conf);
                main_events.push(Event::Start(Tag::Link(kind, link, title)))
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if !lang.is_empty() => {
                let Some(Event::Text(source)) = parser.next() else {
                    panic!()
                };
                let Some(Event::End(Tag::CodeBlock(_))) = parser.next() else {
                    panic!()
                };

                for chr in lang.chars() {
                    assert!(
                        chr.is_ascii_alphanumeric(),
                        "Invalid char {chr:?} in language {lang:?}"
                    );
                }

                let html = crate::highlight::highlight(&lang, &source);

                let opening = format!("<pre lang=\"{lang}\"><code>");

                main_events.push(Event::Html(CowStr::Boxed(opening.into_boxed_str())));
                main_events.push(Event::Html(CowStr::Boxed(html.into_boxed_str())));
                main_events.push(Event::Html(CowStr::Borrowed("</code></pre>")));
            }

            Event::FootnoteReference(name) => {
                let len = numbers.len() + 1;
                let number = *numbers.entry(name.clone()).or_insert(len);

                assert_eq!(number, len); // TODO: handle duplicate footnote references

                let s = format!(
                    "<sup class=\"footnote-reference\" id=\"fnref:{number}\"><a href=\"#fn:{number}\">{number}</a></sup>"
                )
                .into_boxed_str();

                main_events.push(Event::Html(CowStr::Boxed(s)));
                names.push(name);

                assert_eq!(number, names.len());
            }
            _ => {
                main_events.push(event);
            }
        }
    }

    html::push_html(&mut out, main_events.into_iter());

    if footnotes.is_empty() {
        return out;
    }

    out.push_str("<hr/><ol>");

    for (n, fnname) in names.iter().enumerate() {
        let n = n + 1;
        // out.push_str("<li id=\"fn:{n}\">");
        write!(out, "<li id=\"fn:{n}\">").unwrap();

        let backlink = format!("<a href=\"#fnref:{n}\" class=\"footnote-backref\">↩</a>");

        // let mut fn_events = footnotes[fnname].to_owned();
        let Some(mut fn_events) = footnotes.get(fnname).cloned() else {
            panic!("Missing footnote {fnname:?}")
        };

        assert_eq!(
            fn_events.last(),
            Some(&Event::End(Tag::Paragraph)),
            "Unexpected footnote content {fn_events:?}"
        );
        let backlink = Event::Html(CowStr::Borrowed(&backlink));
        fn_events.insert(fn_events.len() - 1, backlink);

        html::push_html(&mut out, fn_events.into_iter());
        out.push_str("</li>\n");
    }

    out.push_str("\n</ol>\n");

    out
}

fn fix_link<'a>(link: CowStr<'a>, conf: &Config) -> CowStr<'a> {
    if let Some(link) = link.strip_prefix('/') {
        CowStr::Boxed(format!("{}{}", conf.base_url, link).into_boxed_str())
    } else {
        link
    }
}

fn collect_footnote<'a>(p: &mut Parser<'a, '_>) -> Vec<Event<'a>> {
    let mut depth = 1;
    let mut events = Vec::new();
    while let Some(e) = p.next() {
        match e {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return events;
                }
            }
            _ => {}
        };
        events.push(e);
    }

    unreachable!("footnote should be terminated by balanced End")
}

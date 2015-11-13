extern crate handlebars; 
use handlebars::{
	Context,
	Handlebars,
	JsonRender,
	Renderable,
	RenderContext,
	RenderError,
	Helper
	};

extern crate pulldown_cmark;
use self::pulldown_cmark::Parser;
use self::pulldown_cmark::html;

use std::io::prelude::*;

pub fn render_error(desc: &'static str) -> RenderError {
    RenderError {
        desc: desc
    }
}

pub fn render_html(text: String) -> String {
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = Parser::new(&*text);
    html::push_html(&mut s, p);
    s
}

pub fn markdown_helper(c: &Context, h: &Helper, _ : &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
	let markdown_text_var = try!(h.param(0).ok_or_else(|| render_error("Param not found for helper \"markdown\"")));
	let markdown_text = c.navigate(rc.get_path(), &markdown_text_var).render(); 
	let html_string = render_html(markdown_text);
	try!(rc.writer.write(html_string.into_bytes().as_ref()));
	Ok(())
}

#[cfg(test)]
mod test {
    use handlebars::{Template, Handlebars};
    use std::collections::BTreeMap;

    #[test]
    fn test_markdown() {
        let t0 = Template::compile("{{markdown x}}".to_string()).ok().unwrap();

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("markdown", Box::new(::markdown_helper));
        handlebars.register_template("t0", t0);

        let mut m :BTreeMap<String, String> = BTreeMap::new();
        m.insert("x".into(), "# wow\n\n## second wow".into());

        let r0 = handlebars.render("t0", &m);
        assert_eq!(r0.ok().unwrap(), "<h1>wow</h1>\n<h2>second wow</h2>\n".to_string());
    }
}

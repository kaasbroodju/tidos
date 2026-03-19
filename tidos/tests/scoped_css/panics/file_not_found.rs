use tidos::{scoped_css, Page};

fn file_not_found() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let _class = scoped_css!("./nonexistent.css");
}

fn main() {}

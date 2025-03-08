pub(crate) use crate::page::Page;
pub trait Component {
	fn to_render(&self,page: &mut Page) -> String;
}

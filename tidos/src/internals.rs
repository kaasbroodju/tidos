use std::borrow::Cow;

#[inline]
pub fn sanitize<S: AsRef<str> + ?Sized>(input: &S) -> String {
	let input = input.as_ref();

	if !input.contains(['&', '<', '>', '"', '\'']) {
		return String::from(input)
	}

	let mut result = String::new();

	for c in input.chars() {
		match c {
			'&' => result.push_str("&amp;"),
			'<' => result.push_str("&lt;"),
			'>' => result.push_str("&gt;"),
			'"' => result.push_str("&quot;"),
			'\'' => result.push_str("&#x27;"),
			_ => result.push(c),
		}
	}

	result
}

use std::borrow::Cow;

/// Sanitize `input` for safe HTML output.
///
/// Returns `Cow::Borrowed(input)` — **zero allocation** — when no characters
/// need escaping.  Only allocates an owned `String` when one of `& < > " '`
/// is present and needs to be replaced with its HTML entity.
#[inline]
pub fn sanitize<S: AsRef<str> + ?Sized>(input: &S) -> Cow<'_, str> {
	let input = input.as_ref();

	if !input.contains(['&', '<', '>', '"', '\'']) {
		return Cow::Borrowed(input);
	}

	let mut result = String::with_capacity(input.len());

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

	Cow::Owned(result)
}

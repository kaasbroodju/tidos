#[inline]
pub fn sanitize(content: String) -> String {
	content
		.chars()
		.fold(String::with_capacity(content.len()), |mut acc, c| {
			acc + match c {
				'&' => { "&amp;" }
				'<' => { "&lt;" }
				'>' => { "&gt;" }
				'"' => { "&quot;" }
				'\'' => { "&#x27;" }
				_ => {
					// This buffer is only used for non-ASCII characters.
					static mut BUFFER: [u8; 4] = [0; 4];

					// Check if the character is ASCII.
					if c.is_ascii() {
						// SAFETY: `c` is ASCII, so it's guaranteed to be a single byte and valid UTF-8.
						let byte = c as u8;
						unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(&byte as *const u8, 1)) }
					} else {
						// Non-ASCII characters need to be encoded into UTF-8.
						unsafe {
							let len = c.encode_utf8(&mut BUFFER).len();
							std::str::from_utf8_unchecked(std::slice::from_raw_parts(BUFFER.as_ptr(), len))
						}
					}
				}
			}
		})
}
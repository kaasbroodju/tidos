use tidos::{view, Component, Page};

fn render(f: impl FnOnce(&mut Page)) -> String {
	let mut p = Page::new();
	f(&mut p);
	p.into_html()
}

#[test]
fn a_simple_toggle_attribute() {
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" checked />
			}
		}),
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" :checked={ true } />
			}
		}),
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" :checked={ false } />
			}
		}),
		r#"<input type="radio" name="day" value="monday" />"#
	);
}

// Attributes are now rendered in source order.
#[test]
fn a_simple_toggle_attribute_source_order() {
	assert_eq!(
		render(|page| {
			view! {
				<input :checked={true} type="radio" name="day" value="monday" />
			}
		}),
		r#"<input checked type="radio" name="day" value="monday" />"#
	);
}

#[test]
fn toggle_attribute_implicit_variable_name() {
	let checked = true;
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" :checked />
			}
		}),
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	let checked = false;
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" :checked />
			}
		}),
		r#"<input type="radio" name="day" value="monday" />"#
	);
}

#[test]
fn missing_variable_defined_toggle() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/attributes/panics/missing_variable_defined_toggle.rs");
}

#[test]
fn invalid_toggle_value() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/attributes/panics/invalid_toggle_value.rs");
}

#[test]
fn attribute_value_without_delimiters() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/attributes/panics/attribute_value_without_delimiters.rs");
}

#[test]
fn data_attribute() {
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" data-tidos={ String::from("css-420")} />
			}
		}),
		r#"<input type="radio" name="day" value="monday" data-tidos="css-420" />"#
	);
}

#[test]
fn attribute_literal_in_braces() {
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" data-tidos={"css-420"} />
			}
		}),
		r#"<input type="radio" name="day" value="monday" data-tidos="css-420" />"#
	);
}

#[test]
fn attribute_formatted_string() {
	let variant = 420;
	assert_eq!(
		render(|page| {
			view! {
				<input type="radio" name="day" value="monday" data-tidos={"css-{}", variant} />
			}
		}),
		r#"<input type="radio" name="day" value="monday" data-tidos="css-420" />"#
	);
}

#[test]
fn custom_element_all_attribute_styles() {
	let active = true;
	let label = String::from("hello");

	// Attributes are rendered in source order.
	assert_eq!(
		render(|page| {
			view! {
				<my-widget
					class="wrapper"
					data-id={ label }
					disabled
					:active={ true }
					:active
				/>
			}
		}),
		r#"<my-widget class="wrapper" data-id="hello" disabled active active />"#
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn struct_component_all_prop_styles() {
	struct Widget {
		pub class: &'static str,
		pub label: String,
		pub disabled: bool,
		pub active: bool,
		pub visible: bool,
	}

	impl Component for Widget {
		fn to_render(&self, page: &mut Page) {
			let s = format!(
				"class={} label={} disabled={} active={} visible={}",
				self.class, self.label, self.disabled, self.active, self.visible,
			);
			page.push_dynamic(s);
		}
	}

	let visible = true;
	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Widget
			class="wrapper"
			label={ String::from("hello") }
			disabled
			:active={ true }
			:visible
		/>
	};

	assert_eq!(
		page_output.into_html(),
		"class=wrapper label=hello disabled=true active=true visible=true"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn struct_component_literal_prop() {
	struct Widget {
		pub class: &'static str,
	}

	impl Component for Widget {
		fn to_render(&self, page: &mut Page) {
			page.push_dynamic(format!("class={}", self.class));
		}
	}

	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Widget class={"wrapper"} />
	};

	assert_eq!(page_output.into_html(), "class=wrapper");
}

#[cfg(not(feature = "i18n"))]
#[test]
fn struct_component_formatted_prop() {
	struct Widget {
		pub label: String,
	}

	impl Component for Widget {
		fn to_render(&self, page: &mut Page) {
			page.push_dynamic(format!("label={}", self.label));
		}
	}

	let variant = 420;
	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Widget label={"css-{}", variant} />
	};

	assert_eq!(page_output.into_html(), "label=css-420");
}

#[cfg(not(feature = "i18n"))]
#[test]
fn struct_component_default() {
	#[derive(Default)]
	struct Coordinate {
		pub x: usize,
		pub y: usize,
	}

	impl Component for Coordinate {
		fn to_render(&self, page: &mut Page) {
			view! {
				<span>{self.x.to_string()}</span><span>{self.y.to_string()}</span>
			}
		}
	}

	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Coordinate
			x={1}
			..
		/>
	};

	assert_eq!(page_output.into_html(), "<span>1</span><span>0</span>");
}

#[test]
fn default_flag_on_native_element() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/attributes/panics/default_flag_on_native_element.rs");
}

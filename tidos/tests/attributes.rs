use tidos::{view, Component, Page};

#[test]
fn a_simple_toggle_attribute() {

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" checked />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked={ true } />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked={ false } />
		},
		r#"<input type="radio" name="day" value="monday" />"#
	);
}

// This test is for performance.
#[test]
fn a_simple_toggle_attribute_reordered() {
	assert_eq!(
		&view!{
			<input :checked={true} type="radio" name="day" value="monday" />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);
}

#[test]
fn toggle_attribute_implicit_variable_name() {

	let checked = true;
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	let checked = false;
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked />
		},
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
		&view!{
			<input type="radio" name="day" value="monday" data-octo={ String::from("css-420")} />
		},
		r#"<input type="radio" name="day" value="monday" data-octo="css-420" />"#
	);
}

#[test]
fn custom_element_all_attribute_styles() {
	let active = true;
	let label = String::from("hello");

	assert_eq!(
		&view! {
			<my-widget
				class="wrapper"
				data-id={ label }
				disabled
				:active={ true }
				:active
			/>
		},
		r#"<my-widget class="wrapper" disabled data-id="hello" active active />"#
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn struct_component_all_prop_styles() {
	struct Widget {
		pub class: &'static str,  // ConstantLiteral:  class="wrapper"
		pub label: String,        // ConstantGroup:     label={ expr }
		pub disabled: bool,       // Constant:          disabled  →  true
		pub active: bool,         // ExplicitToggle:    :active={ true }
		pub visible: bool,        // ImplicitToggle:    :visible  (uses variable)
	}

	impl Component for Widget {
		fn to_render(&self, _page: &mut Page) -> String {
			format!(
				"class={} label={} disabled={} active={} visible={}",
				self.class, self.label, self.disabled, self.active, self.visible,
			)
		}
	}

	let visible = true;
	let mut page_output = Page::new();
	let page = &mut page_output;

	let result = view! {
		<Widget
			class="wrapper"
			label={ String::from("hello") }
			disabled
			:active={ true }
			:visible
		/>
	};

	assert_eq!(result, "class=wrapper label=hello disabled=true active=true visible=true");
}
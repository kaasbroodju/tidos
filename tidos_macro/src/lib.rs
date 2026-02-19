extern crate proc_macro;

use proc_macro::TokenStream;

use crate::page_wrapper::{I18nHoist, PageWrapper};
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use tokens::Component;
use uuid::Uuid;
use crate::i18n::I18n;

mod code_generation;
mod page_wrapper;
mod parsing;
mod tokens;
mod i18n;

/// Turn your template into a usable string
///
/// # Examples
/// ## A for loop
/// ```rust,no_run
///use tidos_macro::view;
///let names = vec!["Bob", "Alice"];
///
///view! {
///    {#for name in names}
///        <p>{format!("Hello {}!", name)}</p>
///    {/for}
///}
/// ```
/// ## A match statement
/// ```rust,no_run
/// enum Pet {
///     Fish,
///     Dog,
///     Cat,
///     Other { name: String }
/// }
///
/// use tidos_macro::view;
/// use Pet::*;
///
/// let my_pet = Dog;
///
/// view! {
///     {#match my_pet}
///         {:case Fish}
///             <p>Blub!</p>
///         {:case Dog}
///             <p>Who's a good boy!</p>
///         {:case Cat}
///             <p>Give all mortal possessions to cat!</p>
///         {:case _}
///             <p>Is it a snake or a spider?</p>
///     {/match}
/// }
/// ```
///
/// ## If/else statements
/// ```rust,no_run
///use tidos_macro::view;
///let age = 18;
///
///let is_american = false;
///
///view! {
///    {#if age >= 18 && !is_american}
///        <p>User is allowed to drink.</p>
///    {:else if age >= 21 && is_american}
///        <p>User is allowed to drink.</p>
///    {:else if age >= 18 && age < 21 && is_american}
///        <p>User is probably designated driver.</p>
///    {:else}
///        <p>User is not allowed to drink.</p>
///    {/if}
///}
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Component);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Similar to the view, however it will also render the templates into a page context.
///
/// # Example
/// ```rust,no_run
/// pub fn getting_started() -> Page {
///     let x: isize = Default::default();
///
///     page! {
///         <main>
///             <h1>Getting started</h1>
///             <p>Cargo add</p>
///             <p>{x.to_string()}</p>
///         </main>
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn page(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as PageWrapper);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Adds body contents to the head of the page
///
/// # example
/// ```rust,no_run
///use tidos_macro::{head, view};
///
///pub struct Title {
///    pub title: String,
///}
///
///impl Component for Title {
///    fn to_render(&self, page: &mut Page) -> String {
///        head! {
///            <title>{&self.title}</title>
///        }
///        String::new()
///    }
///}
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Component);

	let x = Uuid::new_v4().to_string();
	// todo refactor it to &str when I'm comfortable with lifetime annontations
	let input = quote!(
		page.add_elements_to_head(#x, #input);
	);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// CSS file to include in the head of the body.
/// Macro returns a CSS class you can use in your view template
///
/// # Example
/// ```rust,no_run
/// use tidos_macro::{scoped_css, view};
///
/// pub struct DocsGrid {
///     pub content: String
/// }
///
/// impl Component for DocsGrid {
///     fn to_render(&self, page: &mut Page) -> String {
///         let css = scoped_css!("./mod.css");
///
///
///         view! {
///             <div class={css}>
///                 <div>
///                     <DocsSidenav />
///                     <main>
///                         @html{&self.content}
///                     </main>
///                     <DocsOnThisPage />
///                 </div>
///             </div>
///
///         }
///     }
/// }
/// ```
///
#[allow(clippy::all)]
#[proc_macro]
pub fn scoped_css(input: TokenStream) -> TokenStream {
	let file_name = input.to_string().replace("\"", "");

	let x = format!("tidos-{}", Uuid::new_v4().to_string());
	// todo refactor it to &str when I'm comfortable with lifetime annontations
	let input = quote!(
		{
			page.add_elements_to_head(#x, String::from(concat!("<style>.", #x, " {", include_str!(#file_name), "}</style>")));
			#x
		}
	);

	let expanded = input.to_token_stream();

	expanded.into()
}

#[allow(clippy::all)]
#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as I18n);

	let expanded = input.to_token_stream();

	expanded.into()
}

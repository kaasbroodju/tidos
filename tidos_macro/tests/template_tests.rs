use macros::view;

use crate::Pet::*;

macro_rules! template {
    // match statement
    {{#match $i:ident}$({:case $case_statement:pat}$content:literal)*{/match}} => {
        match $i {
            $($case_statement => {
                println!("{}", $content);
            },)*
        }

    };
    // if statement
    {{#if $statement:expr} $($case_content:literal)* {/if}} => {
        if $statement {
            println!("{}", $($case_content)*);

        }
    };
    // if with repeating else if statements
    {{#if $if_expr:expr} $($if_content:literal)* $({:else if $case_expr:expr} $($case_content:literal)*)* {/if}} => {
        if $if_expr {
            println!("{}", $($if_content)*);
        } $(else if $case_expr {
            println!("{}", $($case_content)*);
        })+
    };
    // if with repeating else if statements  with closing else statement
    {{#if $if_expr:expr} $($if_content:literal)* $({:else if $case_expr:expr} $($case_content:literal)*)+ {:else} $($else_case_content:literal)* {/if}} => {
        if $if_expr {
            println!("{}", $($if_content)*);
        } $(else if $case_expr {
            println!("{}", $($case_content)*);
        })+ else {
            println!("{}", $($else_case_content)*);
        }
    };
    // if with else statement
    {{#if $statement:expr} $($if_case_content:literal)* {:else} $($else_case_content:literal)* {/if}} => {
        if $statement {
            println!("{}", $($if_case_content)*);
        } else {
            println!("{}", $($else_case_content)*);
        }
    };
    {{$tag:ident}$content:literal{/$closing_tag:ident}} => {
        // Use the captured literal content in the expansion
        println!("{}", $content);
    };
    {<p> $($ignore:tt)* </p>} => {};
}

#[derive(Debug)]
enum Pet {
	Fish,
	Dog,
	Cat,
	Other { name: String }
}

#[test]
fn it_works() {


	let is_american = true;
	let age = 55;
	let my_pet = Dog;


	template! {
        {#match my_pet}
            {:case Fish}
                "Blub!"
            {:case Dog}
                "Who 's a good boy!"
            {:case Cat}
                "Give al mortal possessions to cat!"
            {:case _}
                "Is it a snake or a spider?"
        {/match}
    };

	template! {
        {#if age >= 21 && is_american == true}
            "<p>allowed to join the military and drink</p>"
        {/if}
    }

	template! {
        {#if false}
            "<p>allowed to join the military and drink</p>"
        {:else}
            "<p>Go back to school</p>"
        {/if}
    }

	template! {
        {#if false}
            "<p>allowed to join the military and drink</p>"
        {:else if true}
            "<p>allowed to join the military</p>"
        {/if}
    }

	template! {
        {#if false}
            "<p>allowed to join the military and drink</p>"
        {:else if false}
            "<p>allowed to join the military</p>"
        {:else if true}
            "<p>allowed to join the military and drink</p>"
        {/if}
    }

	template! {
        {#if false}
            "<p>allowed to join the military and drink</p>"
        {:else if false}
            "<p>allowed to join the military</p>"
        {:else if false}
            "<p>allowed to join the military and drink</p>"
        {:else}
            "<p>Go back to school</p>"
        {/if}
    }

	template! {
        {#if false}
            "<p>allowed to join the military and drink</p>"
        {:else if false}
            "<p>allowed to join the military</p>"
        {:else if true}
            "<p>allowed to join the military and drink</p>"
        {:else}
            "<p>Go back to school</p>"
        {/if}
    }

	expan
	view! {
        <p>hello</p>
    }



	// template! {
	//     {#if age >= 21 && is_american == true}
	//         // <p>allowed to join the military and drink</p>
	//     {:else if age >= 18 && is_american == true}
	//         // <p>allowed to join the military</p>
	//     {:else if age >= 18 && is_american == false}
	//         // <p>allowed to join the military and drink</p>
	//     {:else}
	//         // <p>Go back to school</p>
	//     {/if}
	// }
}

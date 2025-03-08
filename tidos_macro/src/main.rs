// use macros::view;
//
// use crate::components::*;
//
// mod components;
//
// fn main() {
//     enum Pet {
//         Fish,
//         Dog,
//         Cat,
//         // Other,
//         Other { name: String }
//     };
//     let my_pet = Pet::Other { name: String::from("goat") };
//     let numbers = vec![1, 2, 3];
//     let name = "Morris";
//
//     let is_american = true;
//     let age = 3;
//
//     // let x = my_macro! {
//     //     <p>hello</p>
//     //     <p>{ format!("hello: {}", name ) }</p>
//     //     {#for x in &numbers}
//     //         <p>{ x }</p>
//     //     {/for}
//     // };
//     // let x = my_macro! {
//     //     {#for x in &numbers}
//     //         <p>{x}</p>
//     //     {/for}
//     // };
//
//     // let x = my_macro! {
//     //     {#match my_pet}
//     //         {:case Pet::Fish}
//     //             <p>it is a fish</p>
//     //         {:case Pet::Dog}
//     //             <p>Who is a good boy</p>
//     //         {:case Pet::Cat}
//     //             <p>Give al mortal possessions to cat</p>
//     //         {:case Pet::Other { name } }
//     //             <p> { name } </p>
//     //     {/match}
//     // };
//
//     // let x = view! {
//     //     {#if age >= 21 && is_american }
//     //         <p>allowed to join the military and drink</p>
//     //     {:else if age >= 18 && is_american}
//     //         <p>allowed to join the military</p>
//     //     {:else if age >= 18 && !is_american}
//     //         <p>allowed to join the military and drink</p>
//     //     {:else}
//     //         <p>Go back to school</p>
//     //     {/if}
//     // };
//
//     // println!("{}", x);
//     // println! ("<p>{}</p>", "hello") ;
//
//     let x = view! {
//         <img src="logo.svg" />
//     };
//
//     println!("{}", x);
//
//     let x = view! {
//         <HelloWorld name="logo.svg" />
//     };
//
//     println!("{}", x);
//
//
//     // HelloWorld { name: String::from("Morris") }.to_render();
//     // println!("{}", HelloWorld { name: String::from("Morris") }.to_render())
//
// }
//
// // pub struct TestComponent;
// //
// // impl Component for TestComponent {
// //     fn to_render(&self) -> String {
// //         view! {
// //             <HelloWorld></HelloWorld>
// //         }
// //     }
// // }
//
// pub struct HelloWorld<'a> {
//     pub name: &'a str
// }
//
// impl Component for HelloWorld<'_> {
//     fn to_render(&self) -> String {
//         view! {
//             <p>{ format!("hello {}", &self.name) }</p>
//         }
//     }
// }

#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
struct LikesProps {
    score: i32,
}

fn Likes(cx: Scope<LikesProps>) -> Element {
    cx.render(rsx! {
        div {
            "This post has ",
            b { "{cx.props.score}" },
            " likes"
        }
    })
}

#[derive(PartialEq, Props)]
struct IntoProps {
    #[props(into)]
    string: String,
}

fn IntoComponent(cx: Scope<IntoProps>) -> Element {
    cx.render(rsx!(h1 { "{cx.props.string}" }))
}

// create a component that renders a div with the text "Hello, world!"
// fn App(cx: Scope) -> Element {
//     cx.render(rsx! {
//         Likes {
//             score: 42,
//         },
//         IntoComponent{
//             string: "fsfd"
//         }
//     })
// }
// fn App(cx: Scope) -> Element {
//     let list = use_ref(cx, Vec::new);
//
//     cx.render(rsx!(
//         p { "Current list: {list.read():?}" }
//         button {
//             onclick: move |event| {
//                 list.with_mut(|list| list.push(event.value.clone()));
//             },
//             "Click me!"
//         }
//     ))
// }

// fn App(cx: Scope) -> Element {
//     let name = use_state(cx, || "bob".to_string());
//
//     cx.render(rsx! {
//         input {
//             // we tell the component what to render
//             value: "{name}",
//             // and what to do when the value changes
//             oninput: move |evt| name.set(evt.value.clone()),
//         }
//         div {
//             "{name}"
//         }
//     })
// }

// fn App(cx: Scope) -> Element {
//     cx.render(rsx! {
//         form {
//             onsubmit: move |event| {
//                 println!("Submitted! {event:?}")
//             },
//             input { name: "name", },
//             input { name: "age", },
//             input { name: "date", },
//             input { r#type: "submit", },
//         }
//     })
// }

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
            div {
    class: "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8",
    div {
        class: "sm:mx-auto sm:w-full sm:max-w-sm",
        img {
            class: "mx-auto h-10 w-auto",
            alt: "Your Company",
            src: "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600",
        }
        h2 {
            class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900",
            "Sign in to your account"
        }
    }
    div {
        class: "mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
        form {
            class: "space-y-6",
            action: "#",
            method: "POST",div {
                label {
                    class: "block text-sm font-medium leading-6 text-gray-900",
                    r#for: "email","Email address"
                }
                div {
                    class: "mt-2",
                    input {
                        id: "email",
                        autocomplete: "email",
                        name: "email",
                        class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                        r#type: "email",
                    }
                }
            }
            div {
                div {
                    class: "flex items-center justify-between",
                    label {
                        class: "block text-sm font-medium leading-6 text-gray-900",
                        r#for: "password","Password"
                    }
                    div {
                        class: "text-sm",
                        a {
                            class: "font-semibold text-indigo-600 hover:text-indigo-500",
                            href: "#","Forgot password?"
                        }
                    }
                }
                div {
                    class: "mt-2",
                    input {
                        id: "password",
                        autocomplete: "current-password",
                        name: "password",
                        class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                        r#type: "password",
                    }
                }
            }
            div {
                button {
                    class: "flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    r#type: "submit","Sign in"
                }
            }
        }
        p {
            class: "mt-10 text-center text-sm text-gray-500",
            "Not a member?"
            a {
                class: "font-semibold leading-6 text-indigo-600 hover:text-indigo-500",
                href: "#","Start a 14 day free trial"
            }
        }
    }
}
    })
}

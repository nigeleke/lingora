use dioxus::prelude::{document::*, *};

#[component]
pub fn Translation() -> Element {
    // let app = use_context::<Signal<CoreApp>>();

    let identifier_name = use_signal(String::new);
    // let mut reference = use_signal(|| None);
    // let mut target = use_signal(|| None);

    // use_effect(move || {
    //     let identifier = app.read().selected_identifier().cloned();
    //     identifier_name.set(identifier.map(|i| i.name()).unwrap_or_default());
    //     reference.set(app.read().reference_translation());
    //     target.set(app.read().target_translation());
    // });

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/translation.css") }
        div {
            class: "translation",
            p { strong { {identifier_name} } }
            // TranslationComparison {
            //     reference: reference(),
            //     target: target()
            // }
        }
    }
}

// #[component]
// fn TranslationComparison(
//     reference: Option<CoreTranslation>,
//     target: Option<CoreTranslation>,
// ) -> Element {
//     const REFERENCE: &str = "Reference";
//     const TARGET: &str = "Target";

//     let extract_comparators = |default_header: &str, translation: Option<CoreTranslation>| {
//         let defaults = (Err(default_header.into()), Err("".into()));
//         translation.map_or(defaults, |t| {
//             (
//                 Ok(default_header.into()),
//                 Ok(t.entry().to_string()), // Ok(format!("{}{}{}", pattern, separator, attributes)),
//             )
//         })
//     };

//     let (left_header, left_arguments) = extract_comparators(REFERENCE, reference);
//     let (right_header, right_arguments) = extract_comparators(TARGET, target);

//     rsx! {
//         div {
//             class: "translation-comparison",
//             SideBySide { left: left_header, right: right_header, }
//             SideBySide { left: left_arguments, right: right_arguments, }
//         }
//     }
// }

// #[component]
// fn SideBySide(left: Result<String, String>, right: Result<String, String>) -> Element {
//     let left_text = left.as_ref().unwrap_or_else(|_| left.as_ref().unwrap_err());
//     let right_text = right
//         .as_ref()
//         .unwrap_or_else(|_| right.as_ref().unwrap_err());

//     rsx! {
//         div {
//             class: "lhs",
//             class: if left.is_err() { "quietly" },
//             {left_text.clone()}
//         }
//         span {}
//         div {
//             class: "rhs",
//             class: if right.is_err() { "quietly" },
//             {right_text.clone()}
//         }
//     }
// }

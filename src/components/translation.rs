use crate::core::prelude::{CoreApp, CoreTranslation};

use dioxus::prelude::{document::*, *};

#[component]
pub fn Translation() -> Element {
    let app = use_context::<Signal<CoreApp>>();

    let mut identifier_name = use_signal(String::new);
    let mut reference = use_signal(|| None);
    let mut target = use_signal(|| None);

    use_effect(move || {
        let identifier = app.read().selected_identifier().cloned();
        identifier_name.set(identifier.map(|i| i.name()).unwrap_or_default());
        reference.set(app.read().reference_translation());
        target.set(app.read().target_translation());
    });

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/translation.css") }
        div {
            class: "translation",
            p { strong { {identifier_name} } }
            TranslationComparison { reference, target }
        }
    }
}

#[component]
fn TranslationComparison(
    reference: Signal<Option<CoreTranslation>>,
    target: Signal<Option<CoreTranslation>>,
) -> Element {
    let mut reference_text = use_signal(String::new);

    use_effect(move || {
        reference_text.set(
            reference
                .read()
                .as_ref()
                .map_or("".to_string(), |reference| reference.to_string()),
        );
    });

    let mut target_text = use_signal(String::new);

    use_effect(move || {
        target_text.set(
            target
                .read()
                .as_ref()
                .map_or("".to_string(), |target| target.to_string()),
        )
    });

    rsx! {
        div {
            p { {reference_text()} }
            p { {target_text()} }
        }
    }
}

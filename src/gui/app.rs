use dioxus::prelude::{document::*, *};

use super::{components::Workspace, state::State};
use crate::{config::Settings, domain::Analysis};

#[component]
pub fn App(settings: Settings, analysis: Analysis) -> Element {
    let settings = use_signal(|| settings);
    let analysis = use_signal(|| analysis);

    provide_context(settings);
    provide_context(analysis);

    let state = State::from(&*settings.read());
    let state = use_signal(|| state);
    provide_context(state);

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        Workspace { }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        config::Settings,
        domain::{Analysis, IntegrityChecks, Locale},
    };

    #[test]
    fn app_is_rendered() {
        let mut vdom = VirtualDom::new_with_props(App, {
            let settings = Settings::try_from_str(
                Locale::from_str("en-GB").unwrap(),
                r#"
[lingora]
reference = "tests/data/i18n/en/en-GB.ftl"
targets = ["tests/data/i18n/"]
"#,
            )
            .unwrap();
            let checks = IntegrityChecks::try_from(&settings).unwrap();
            let analysis = Analysis::from(checks);
            AppProps { settings, analysis }
        });

        vdom.rebuild_in_place();

        let html = dioxus::ssr::render(&vdom);
        insta::assert_snapshot!(html, @r#"<div class="workspace"><section class="workspace-item"><div class="scrollable"><div class="languages-tree"><ul><li>en</li><ul><li class="  ok" tabindex="0" role="button"><div><span>en</span><span></span></div></li><li class="  ok" tabindex="0" role="button"><div><span>en-AU</span><span></span></div></li><li class=" reference ok" tabindex="0" role="button"><div><span>en-GB</span><span></span></div></li></ul><li>it</li><ul><li class="  ok" tabindex="0" role="button"><div><span>it-IT</span><span></span></div></li></ul></ul></div></div></section><section class="workspace-item"><div class="identifiers-outer"><div class="identifiers-filter"><input type="text" placeholder="🔎"/><label class="both"><input type="checkbox"/><span>●</span><span>Ok</span></label><label class="superfluous-target"><input type="checkbox"/><span>●</span><span>Warning</span></label><label class="missing-target"><input type="checkbox"/><span>●</span><span>Error</span></label></div><div class="scrollable"><div class="identifiers-tree"><ul></ul></div></div></div></section><section class="workspace-item"><div class="translation"><p><strong></strong></p><div class="translation-comparison"><div class="lhs "><div class="scrollable">Reference</div></div><span></span><div class="rhs "><div class="scrollable">Target</div></div><div class="lhs translation-content"><div class="scrollable"></div></div><span></span><div class="rhs translation-content"><div class="scrollable"></div></div></div></div></section><section class="workspace-item"><div class="warnings"><div class="scrollable"><div class="warnings-panel"></div></div></div></section></div><div class="status"><div>./i18n/</div><div class="status-error"></div><span>en-GB ⤅ ≪no target≫</span></div>"#);
    }
}

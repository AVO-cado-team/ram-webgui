use yew::function_component;
use yew::prelude::*;

#[function_component]
pub fn HydrationGate(props: &Props) -> Html {
    let is_hydrated = use_state(|| false);
    let is_hydrated_cloned = is_hydrated.clone();

    use_effect_with_deps(move |_| is_hydrated_cloned.set(true), ());

    if *is_hydrated {
        html! { for props.children.iter() }
    } else {
        props.placeholder.clone()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub placeholder: Html,
}

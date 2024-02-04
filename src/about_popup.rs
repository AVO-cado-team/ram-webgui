use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub popup_ref: NodeRef,
}

#[function_component(AboutPopup)]
pub fn about_popup(props: &Props) -> Html {
    let body = gloo::utils::body();

    let ramemu = html! { <a href="https://github.com/AVO-cado-team/ramemu">{"ramemu"}</a> };
    let ram_cli = html! { <a href="https://github.com/AVO-cado-team/ram-cli">{"ram-cli"}</a> };
    let ram_webgui = html! { <a href="https://github.com/AVO-cado-team/ram-webgui">{"ram-webgui"}</a> };

    let popup = html! {
      <div class="about-popup" ref={&props.popup_ref}>
          <div class="about-us-block">
              <div class="about-info-block">
                <h1>{ "About project" }</h1>
                <p>
                    { "If something doesn't sit right with you, you're welcome to contribute via our " }
                    <a href="https://github.com/AVO-cado-team/ram-webgui?tab=readme-ov-file#contributing">{ "contributions page" }</a>
                    { "." }
                </p>
                <div class="authors">
                    <h3>{ "Authors:" }</h3>
                    <br />
                    <div class="author">
                        <a href="https://github.com/Ddystopia" class="author-name">{"Oleksandr Babak "}</a>
                        <div class="author-role"> {ramemu} {", "} {ram_webgui.clone()} </div>
                    </div>
                    <div class="author">
                        <a href="https://github.com/ic-it" class="author-name">{ "Illia Chaban" }</a>
                        <div class="author-role"> {ram_cli.clone()} {", "} {ram_webgui.clone()} </div>
                    </div>
                    <div class="author">
                        <a href="https://github.com/Mykhailo-Sichkaruk" class="author-name">{ "Mykhailo Sichkaruk" }</a>
                        <div class="author-role"> {ram_cli} </div>
                    </div>
                    <div class="author">
                        <a href="https://github.com/0xWraith" class="author-name">{ "Dmytro Dzhuha" }</a>
                        <div class="author-role"> {ram_webgui} </div>
                    </div>
                </div>

                <div class="avo-block">
                    <a href="https://github.com/AVO-cado-team">
                      <img src="assets/github-mark-white.png" alt="avo" class="avo"/>
                    </a>
                    <p>{ "AVO-cado team © 2023" }</p>
                </div>
              </div>
          </div>
      </div>
    };

    create_portal(popup, body.into())
}

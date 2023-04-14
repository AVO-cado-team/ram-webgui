// src/about_popup.rs
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[function_component(AboutPopup)]
pub fn about_popup(_props: &Props) -> Html {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("should have a body on document");

  let popup = html! {
    <div class="about-popup">
        <div class="about-us-block">
            <div class="about-info-block">
              <h1>{ "About project" }</h1>
              <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sit amet dui iaculis velit consectetur sagittis vitae at sem. Aliquam id lacus at nibh accumsan tincidunt et ac eros. Nullam sed malesuada lectus. Nulla sed magna a odio mattis sodales nec in augue. Praesent imperdiet." }</p>
              <div class="authors">
                  <h3>{ "Authors:" }</h3>
                  <br />
                  <div class="author">
                      <a href="https://github.com/Ddystopia" class="author-name">{"Oleksandr Babak "}</a>
                      <div class="author-role">{ "ramemu, ram-webgui"  }</div>
                  </div>
                  <div class="author">
                      <a href="https://github.com/ic-it" class="author-name">{ "Illia Chaban" }</a>
                      <div class="author-role">{ "ram-cli, ram-webgui" }</div>
                  </div>
                  <div class="author">
                      <a href="https://github.com/Mykhailo-Sichkaruk" class="author-name">{ "Mykhailo Sichkaruk" }</a>
                      <div class="author-role">{ "ram-cli" }</div>
                  </div>
                  <div class="author">
                      <a href="https://github.com/0xWraith" class="author-name">{ "Dmytro Dzhuha" }</a>
                      <div class="author-role">{ "ram-webgui" }</div>
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

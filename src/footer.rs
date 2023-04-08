use yew::prelude::*;

const YEW_LINK: &str = "https://yew.rs";
const SOURCE_CODE_LINK: &str = "https://github.com/AVO-cado-team/ram-webgui";
const DEVELOPER_LINK: &str = "https://github.com/AVO-cado-team";

// I think it is a bad idea to include footer, some menu would be better
// but I will leave it for now

#[function_component(Footer)]
pub fn footer() -> Html {
  html! {
    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          { "Built with " }
          <a href={YEW_LINK}>{ "Yew" }</a>
          { " by " }
          <a href={DEVELOPER_LINK} target="_blank">{ "Avocado team" }</a>
          { " Source Code: " }
          <a href={SOURCE_CODE_LINK} target="_blank">{ "Github" }</a>
        </p>
      </div>
    </footer>
  }
}

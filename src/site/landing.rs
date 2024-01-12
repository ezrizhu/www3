use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;

pub async fn landing(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let cloud = state.cloud.clone();

    let (img, img_link, artist) = ("pixel.webp", "https://toyhou.se/StandbySnail", "StandbySnail");
    let img = format!("/assets/img/{}", img);

    let content = html! {
        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" {
                h1 { "Hosted on " a href="./" { "EzriCloud" } "." }
                h3 { "File hosting / Link shortening" }
                a href="https://owo.codes/whats-this/api" { "API Source" }
                br;
                a href="https://owo.codes/whats-this/cdn-origin" { "CDN Source" }
                br;
                h3 { "Email" }
                p { "Powered by " a href="https://www.migadu.com" { "Migadu" } "." }
            }
            div class="pure-u-1 pure-u-md-1-3 hero-img" {
                a target="_blank" href="https://toyhou.se/finnekit" {
                    img class="pure-img" src=(img) alt="EzriCloud's avatar";
                }
                p { "Art by " a target="_blank" href=(img_link) { (artist) } "." }
            }
        }

        h3 { "About" }
        p { "Not-for-profit IT and Networking project that provides free hosting and BGP upstream to students and open-source projects" };

        h3 { "Current Status" }
        p {
            @if cloud.is_down {
                "Down since" (cloud.down_since)
            } @else {
                "All systems operational"
            }
        }
    };

    base(content, state.clone())
}

use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let cloud = state.cloud.clone();

    let (img, img_link, artist) = ("pixel.webp", "https://toyhou.se/StandbySnail", "StandbySnail");

    let img = format!("/assets/img/{}", img);

    let content = html! {
        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" {
                h1 { "EzriCloud (AS206628)" }
                p { "Not-for-profit IT and Networking project that provides free hosting and BGP upstream to students and open-source projects" };
                p { "EzriCloud is a brand of BNS Services LLC, run by " a href="https://ezrizhu.com" target="_blank" { "Ezri Zhu." } }
                p { "Contact info can be found under our " 
                    a href="https://www.peeringdb.com/net/22700" { "PeeringDB page" }
                    ", or under the nic-hdl EZRI-RIPE and ZHUEZ-ARIN" }
            }
            div class="pure-u-1 pure-u-md-1-3 hero-img" {
                a target="_blank" href="https://toyhou.se/finnekit" {
                    img class="pure-img" src=(img) alt="EzriCloud's avatar";
                }
                p { "Art by " a target="_blank" href=(img_link) { (artist) } "." }
            }
        }

        h3 { "Peering Policy" }
        p {
            "I have an open peering policy, and I provide free transit to non-profit and networks."
        }
        h3 { "Special Thanks" }
        p {
            a target="_blank" href="https://he.net" { "Hurricane Electric" }
            " for providing colocation and transit."
            br;
            a target="_blank" href="https://www.nycmesh.net/" { "NYCMesh" }
            " for providing colocation services."
            br;
            a target="_blank" href="https://infernocomms.com/" { "Inferno Communications" }
            " for providing RIPE LIR services and colocation."
            br;
            a target="_blank" href="https://as62513.net/" { "AS835 (Xenyth Cloud)" }
            " for providing ARIN services."
            br;
            a target="_blank" href="https://www.openfactory.net/" { "OpenFactory" }
            " for providing RIPE LIR services."
        }

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

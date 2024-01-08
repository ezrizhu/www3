use maud::{html, Markup};
use crate::SiteState;
pub mod home;

pub fn base(content: Markup, state: SiteState) -> Markup {
    let last_updated = state.last_updated.clone();
    let build_info = format!("Built on: {} • Ref: {} • Commit: {}",
                             std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
                             );
    let description = "EzriCloud - Not-for-profit IT and Networking project.";
    let title = "EzriCloud";

    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    link rel="stylesheet" href="/assets/css/pure-min.css";
                    link rel="stylesheet" href="/assets/css/main.css";
                    link rel="stylesheet" href="/assets/css/grids-responsive-min.css";

                    title { (title) };
                    meta name="description" content=(description);
                    meta name="author" content="Ezri";

                    link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                    link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#4ee06d";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#4ee06d";
                }

                body {
                    div class="main" {
                        (content);
                        div class="footer" {
                            div class="badges" {
                                a target="_blank" href="https://ezri.pet" {
                                    img src="/assets/img/badges/ezri.png" alt="Ezri";
                                }
                                a target="_blank" href="https://as206628.net" {
                                    img src="/assets/img/badges/ezricloud.png" alt="EzriCloud";
                                }
                                a target="_blank" href="https://kernel.org" {
                                    img src="/assets/img/badges/xenia-now.gif" alt="xenia-now";
                                }
                                a target="_blank" href="https://infernocomms.com" {
                                    img src="/assets/img/badges/infernocomms.png" alt="Inferno Communications";
                                }
                                a target="_blank" href="https://glauca.digital/" {
                                    img src="/assets/img/badges/glauca.gif" alt="Glauca Digital";
                                }
                                a target="_blank" href="https://xenyth.net/" {
                                    img src="/assets/img/badges/xenyth.png" alt="xenyth cloud";
                                }
                            }

                            p {
                                "Auto refreshed: " (last_updated)
                                br;
                                "Source code "
                                a target="_blank" href="https://github.com/ezrizhu/www3" { "available here" }
                                ", released under the "
                                a target="_blank" href="https://github.com/ezrizhu/www3/blob/main/COPYING" { "GNU AGPLv3 license" }
                                br;
                                (build_info);
                            }
                        }
                    }

                }
            }
    }
}

use dioxus::prelude::*;

const SPINNER_SCSS: Asset = asset!("/assets/styling/components/spinner.scss");

#[component]
pub fn Spinner() -> Element {
    rsx! {
        document::Stylesheet { href: SPINNER_SCSS }

        div {
            svg {
                class: "spinner-svg",
                width: 140,
                height: 140,
                view_box: "0 0 280 280",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                g {
                    line {
                        x1: 59.9833,
                        y1: 140.333,
                        x2: 219.978,
                        y2: 139,
                        stroke: "#fff",
                        stroke_width: 4,
                        circle {
                            cx: 60,
                            cy: 140,
                            r: 5,
                            fill: "#fff",
                        }
                        circle {
                            cx: 220,
                            cy: 139,
                            r: 5,
                            fill: "#fff",
                        }
                    }
                }

                path {
                    class: "circle",
                    d: "M109.957 122.655L140 105.309L170.043 122.655V157.345L140 174.691L109.957 157.345V122.655Z",
                    stroke: "#fff",
                    stroke_width: 4,
                }
                circle {
                    class: "circle",
                    cx: 140,
                    cy: 140,
                    r: 13,
                    stroke: "#f5f779",
                    stroke_width: 4,
                }

                circle {
                    class: "circle",
                    cx: 110,
                    cy: 192,
                    r: 13,
                    stroke: "#f7a78f",
                    stroke_width: 4,
                }

                circle {
                    class: "circle",
                    cx: 85,
                    cy: 232,
                    r: 8,
                    stroke: "#82c7c5",
                    stroke_width: 4,
                }

                circle {
                    class: "circle",
                    cx: 170,
                    cy: 88,
                    r: 13,
                    stroke: "#82c7f5",
                    stroke_width: 4,
                }

                circle {
                    class: "circle circle-s",
                    cx: 110,
                    cy: 192,
                    r: 5,
                    fill: "#f7a78f",
                }

                circle {
                    class: "circle circle-s",
                    cx: 185,
                    cy: 61,
                    r: 5,
                    fill: "#f5d77b",
                }
            }
        }
    }
}

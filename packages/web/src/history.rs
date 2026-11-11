use std::{rc::Rc, sync::Arc};

use dioxus::prelude::*;

pub struct TrimQueryHistory(Rc<dyn History>);

impl TrimQueryHistory {
    pub fn wrap_current() -> Rc<dyn History> {
        Rc::new(Self(history()))
    }
}

fn without_empty_query(mut route: String) -> String {
    if route.ends_with('?') {
        route.pop();
    }
    route
}

impl History for TrimQueryHistory {
    fn current_route(&self) -> String {
        self.0.current_route()
    }

    fn current_prefix(&self) -> Option<String> {
        self.0.current_prefix()
    }

    fn can_go_back(&self) -> bool {
        self.0.can_go_back()
    }

    fn go_back(&self) {
        self.0.go_back()
    }

    fn can_go_forward(&self) -> bool {
        self.0.can_go_forward()
    }

    fn go_forward(&self) {
        self.0.go_forward()
    }

    fn push(&self, route: String) {
        self.0.push(without_empty_query(route))
    }

    fn replace(&self, path: String) {
        self.0.replace(without_empty_query(path))
    }

    fn external(&self, url: String) -> bool {
        self.0.external(url)
    }

    fn updater(&self, callback: Arc<dyn Fn() + Send + Sync>) {
        self.0.updater(callback)
    }

    fn include_prevent_default(&self) -> bool {
        self.0.include_prevent_default()
    }
}

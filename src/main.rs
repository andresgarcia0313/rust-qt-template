use cstr::cstr;
use qmetaobject::prelude::*;

/// Main component with reactive state
#[derive(QObject)]
struct MainComponent {
    base: qt_base_class!(trait QObject),
    text: qt_property!(QString; NOTIFY text_changed),
    text_changed: qt_signal!(),
    click: qt_method!(fn click(&mut self) {
        self.text = "It works!".into();
        self.text_changed();
    }),
}

impl Default for MainComponent {
    fn default() -> Self {
        Self {
            base: Default::default(),
            text: "Hello World".into(),
            text_changed: Default::default(),
            click: Default::default(),
        }
    }
}

fn main() {
    qml_register_type::<MainComponent>(cstr!("UI"), 1, 0, cstr!("MainComponent"));

    let mut engine = QmlEngine::new();
    engine.load_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui/main.qml").into());
    engine.exec();
}

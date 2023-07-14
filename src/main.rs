use slint::{ModelRc, SharedString};

use crate::highlight::highlight;

mod highlight;

slint::include_modules!();
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
#[allow(unused)]
fn main() {
    let initial_code = "bass = Sample > 10 | Sin(0.5 * t) | Sin(0.5 * t) !\n\nmain = bass + (bass < .2s) + Sin(2t << f)\n\n\nMidi > g";

    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    let do_highlight = move |code: &str| {
        let highlight_tokens = highlight(code);
        let highlight_tokens = ModelRc::new(slint::VecModel::from(highlight_tokens));

        main_window_weak
            .unwrap()
            .set_highlight_tokens(highlight_tokens);
    };

    main_window.set_text(initial_code.into());
    do_highlight(&initial_code);

    main_window.on_edited(move |code: SharedString| {
        do_highlight(&code);
    });

    main_window.run().unwrap();
}

#[allow(unused)]
fn lerp(value: f32, domain: (f32, f32), range: (f32, f32), clamp: bool) -> f32 {
    assert!(range.0 <= range.1);

    let output = (value - domain.0) * (range.1 - range.0) / (domain.1 - domain.0) + range.0;
    if clamp && output < range.0 {
        return range.0;
    } else if clamp && output > range.1 {
        return range.1;
    } else {
        return output;
    }
}

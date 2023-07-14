use slint::{Model, ModelRc, SharedString};
use std::{cell::RefCell, rc::Rc};

use crate::highlight::highlight;

mod highlight;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct HistoryState {
    text: String,
    cursor: (f32, f32),
}

slint::include_modules!();
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    let initial_code = "bass = Sample > 10 | Sin(0.5 * t) | Sin(0.5 * t) | Sin(0.5 * t) !\n\nmain = bass + (bass < .2s) + Sin(2t << f)";

    let history = Rc::new(RefCell::new(vec![HistoryState {
        text: initial_code.into(),
        cursor: (0., 0.),
    }]));

    let main_window = HelloWorld::new().unwrap();

    main_window.set_text(initial_code.into());

    let highlight_tokens = highlight(initial_code);
    let highlight_tokens = ModelRc::new(slint::VecModel::from(highlight_tokens));

    let main_window_weak_1 = main_window.as_weak();

    main_window_weak_1
        .unwrap()
        .set_highlight_tokens(highlight_tokens);

    // main_window.on_cursor_position_changed(move |x: f32, y: f32| {
    //     println!("New cursor position {}, {}", x, y);
    // });

    let history_1 = history.clone();
    main_window.on_edited(move |code: SharedString, x: f32, y: f32| {
        println!("Edited! {}, {}", x, y);

        let highlight_tokens = highlight(&code);
        let highlight_tokens = ModelRc::new(slint::VecModel::from(highlight_tokens));

        main_window_weak_1
            .unwrap()
            .set_highlight_tokens(highlight_tokens);

        let mut h = history_1.borrow_mut();
        h.push(HistoryState {
            text: code.clone().into(),
            cursor: (x, y),
        });
        println!("pushed state, now: {}", h.len());
    });

    let history_2 = history.clone();
    let main_window_weak_3 = main_window.as_weak();
    main_window.on_undo(move || {
        let mut h = history_2.borrow_mut();
        h.pop();
        if let Some(prev) = h.pop() {
            main_window_weak_3
                .unwrap()
                .set_text(prev.text.clone().into());

            // and re-highlight!

            h.push(prev);
        }
    });

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run().unwrap();
}

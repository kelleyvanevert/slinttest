use slint::{Model, ModelRc, SharedString};
use std::rc::Rc;

use crate::highlight::highlight;

mod highlight;

slint::include_modules!();
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    let main_window = HelloWorld::new().unwrap();

    main_window
        .set_text("bass = Sample > 10 | Sin(0.5 * t) | Sin(0.5 * t) | Sin(0.5 * t) !\n\nmain = bass + (bass < .2s) + Sin(2t << f)".into());

    let highlight_tokens = highlight("bass = Sample > 10 | Sin(0.5 * t) | Sin(0.5 * t) | Sin(0.5 * t) !\n\nmain = bass + (bass < .2s) + Sin(2t << f)");
    let highlight_tokens = ModelRc::new(slint::VecModel::from(highlight_tokens));

    let main_window_weak_1 = main_window.as_weak();

    main_window_weak_1
        .unwrap()
        .set_highlight_tokens(highlight_tokens);

    main_window.on_edited(move |str: SharedString| {
        println!("Edited! {:?}", str);

        let highlight_tokens = highlight(&str);
        let highlight_tokens = ModelRc::new(slint::VecModel::from(highlight_tokens));

        main_window_weak_1
            .unwrap()
            .set_highlight_tokens(highlight_tokens);
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

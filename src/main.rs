slint::slint! {
    component MemoryTile inherits Rectangle {
        callback clicked;
        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;

        width: 64px;
        height: 64px;
        background: solved ? #34CE57 : #3960D5;
        animate background { duration: 800ms; }
        border-color: #f7cb05;
        border-width: 1px;
        border-radius: 10px;

        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
            }

        
        Rectangle {
            background: #193076;
            border-color: #193076;
            border-width: 1px;
            border-radius: 10px;
            x: 0px;
            width: open_curtain ? 0px : (parent.width);
            height: parent.height;
            animate width { duration: 500ms; easing: ease-in; }
        }

        // Right curtain
        // Rectangle {
        //     background: #193076;
        //     x: open_curtain ? parent.width : (parent.width / 2);
        //     width: open_curtain ? 0px : (parent.width / 2);
        //     height: parent.height;
        //     animate width { duration: 250ms; easing: ease-in; }
        //     animate x { duration: 250ms; easing: ease-in; }
        // }

        

        TouchArea {
            clicked => {
                // Delegate to the user of this element
                root.clicked();
            }
        }
        
    }

    component PopupExample inherits Rectangle {
        callback clicked;

        border-color: #f7cb05;
        width: 100px;
        height: 100px;

        TouchArea {
            height:100%; width: 100%;
            clicked => { //popup.show(); 
                root.clicked();
            }
        }
    }

    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
    }

    export component MainWindow inherits Window {
        width: 326px;
        height: 326px;

        callback check_if_pair_solved(); // Added
        in property <bool> disable_tiles; // Added

        in property <[TileData]> memory_tiles: [
            { image: @image-url("icons/cancel48.png") },
            { image: @image-url("icons/folder_open.png") },
            { image: @image-url("icons/keyboard_arrow_down.png") },
            { image: @image-url("icons/Keyboard_arrow_up.png") },
            { image: @image-url("icons/save.png") },
            { image: @image-url("icons/aod_tablet.svg") },
            { image: @image-url("icons/box-seam.svg") },
            { image: @image-url("icons/navigate_next.png") },
        ];

        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => { popup.show(); }
            // clicked => {
            //     // old: tile.image_visible = !tile.image_visible;
            //     // new:
            //     if (!root.disable_tiles) {
            //         tile.image_visible = !tile.image_visible;
            //         root.check_if_pair_solved();
            //     }
            // }
        }

        popup := PopupWindow {
            Rectangle { height:100%; width: 100%; background: yellow; }
            x: 20px; y: 20px; height: 50px; width: 50px;
            }
            //clicked => { popup.show(); }

        PopupExample {
            width: 100px;
            height: 100px;
            
        }
        // MemoryTile {
        //     clicked => {
        //     self.open_curtain = !self.open_curtain;
        //     }
        // }
    }
}

#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]

pub fn main() {
    println!("Created by great Samurai Sashima11");
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles =
            tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

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

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use kayak_font::KayakFont;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets { 
    // #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., 
    //         columns = 8, rows = 7, padding_x = 0., padding_y = 0.))]
    // #[asset(path = "art/momcard1.png")]
    // tileset: Handle<TextureAtlas>,
    #[asset(path = "art/Monkv2-2/Monkv2 1.obj")]
    pub monk_idle_0: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 2.obj")]
    pub monk_idle_1: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 3.obj")]
    pub monk_idle_2: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 6.obj")]
    pub monk_walk_1: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 7.obj")]
    pub monk_walk_2: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 8.obj")]
    pub monk_walk_3: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 9.obj")]
    pub monk_walk_4: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 5.obj")]
    pub monk_sit_0: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 10.obj")]
    pub monk_write_1: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 11.obj")]
    pub monk_write_2: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 12.obj")]
    pub monk_write_3: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 13.obj")]
    pub monk_write_4: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 14.obj")]
    pub monk_write_5: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 15.obj")]
    pub monk_write_6: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 16.obj")]
    pub monk_write_7: Handle<Mesh>,
    #[asset(path = "art/Monkv2-2/Monkv2 1.png")]
    pub monk_idle_0_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 2.png")]
    pub monk_idle_1_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 3.png")]
    pub monk_idle_2_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 6.png")]
    pub monk_walk_1_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 7.png")]
    pub monk_walk_2_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 8.png")]
    pub monk_walk_3_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 9.png")]
    pub monk_walk_4_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 5.png")]
    pub monk_sit_0_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 10.png")]
    pub monk_write_1_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 11.png")]
    pub monk_write_2_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 12.png")]
    pub monk_write_3_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 13.png")]
    pub monk_write_4_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 14.png")]
    pub monk_write_5_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 15.png")]
    pub monk_write_6_png: Handle<Image>,
    #[asset(path = "art/Monkv2-2/Monkv2 16.png")]
    pub monk_write_7_png: Handle<Image>,

    #[asset(path = "art/talkingmonk1-Sheet.png")]
    pub monk_face_talk_png: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 64., 
            columns = 8, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "art/talkingmonk1-Sheet.png")]
    pub monk_face: Handle<TextureAtlas>,

    #[asset(path = "art/monkobj/monastery-back2-shelf-1.obj")]
    pub monastery_back_shelf: Handle<Mesh>,
    #[asset(path = "art/monkobj/monastery-back2-shelf-1.png")]
    pub monastery_back_shelf_png: Handle<Image>,

    #[asset(path = "art/wall/MonasteryBack1 1.obj")]
    pub monastery_back_1: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack1 1.png")]
    pub monastery_back_1_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack2 1.obj")]
    pub monastery_back_2: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack2 1.png")]
    pub monastery_back_2_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack3 1.obj")]
    pub monastery_back_3: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack3 1.png")]
    pub monastery_back_3_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack4 1.obj")]
    pub monastery_back_4: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack4 1.png")]
    pub monastery_back_4_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack1-top 1.obj")]
    pub monastery_back_1_top: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack1-top 1.png")]
    pub monastery_back_1_top_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack2-top 1.obj")]
    pub monastery_back_2_top: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack2-top 1.png")]
    pub monastery_back_2_top_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack3-top 1.obj")]
    pub monastery_back_3_top: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack3-top 1.png")]
    pub monastery_back_3_top_png: Handle<Image>,
    #[asset(path = "art/wall/MonasteryBack4-top 1.obj")]
    pub monastery_back_4_top: Handle<Mesh>,
    #[asset(path = "art/wall/MonasteryBack4-top 1.png")]
    pub monastery_back_4_top_png: Handle<Image>,

    #[asset(path = "art/wall/btp-bg1.obj")]
    pub panel_bg1: Handle<Mesh>,
    #[asset(path = "art/wall/btp-bg1.png")]
    pub panel_bg1_png: Handle<Image>,
    #[asset(path = "art/wall/btp-ch1.obj")]
    pub panel_chars1: Handle<Mesh>,
    #[asset(path = "art/wall/btp-ch1.png")]
    pub panel_chars1_png: Handle<Image>,

    #[asset(path = "art/wall/table.obj")]
    pub table: Handle<Mesh>,
    #[asset(path = "art/wall/table.png")]
    pub table_png: Handle<Image>,
    #[asset(path = "art/wall/chair.obj")]
    pub chair: Handle<Mesh>,
    #[asset(path = "art/wall/chair.png")]
    pub chair_png: Handle<Image>,


    #[asset(path = "art/monkobj/monastery-back-1.png")]
    pub monastery_back_png: Handle<Image>,
    #[asset(path = "art/monkobj/greywall-1.obj")]
    pub greywall: Handle<Mesh>,
    #[asset(path = "art/monkobj/greyWall-1.png")]
    pub greywall_png: Handle<Image>,
    #[asset(path = "art/panel-bb.png")]
    pub panel: Handle<Image>,
    #[asset(path = "art/panel--button-bb.png")]
    pub panel_btn: Handle<Image>,
    #[asset(path = "art/panel--button-bb-hover.png")]
    pub panel_btn_hov: Handle<Image>,
    #[asset(path = "art/panel--button-bb-clicked.png")]
    pub panel_btn_clk: Handle<Image>,
    #[asset(path = "ui/pcsenior.kttf")]
    pub kfont: Handle<KayakFont>,
    // #[asset(path = "art/monkv2/Monkv2-1.obj")]
    // pub monk_1_idle_mesh: Handle<Mesh>,

}
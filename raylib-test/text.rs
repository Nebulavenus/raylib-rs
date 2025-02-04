
#[cfg(test)]
mod text_test {
    use raylib::prelude::*;
    use crate::tests::*;
    ray_test!(test_font_load);
    fn test_font_load(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _f = rl
            .load_font(thread, "resources/alagard.png")
            .expect("couldn't load font");
    }

    ray_draw_test!(test_default_font);
    fn test_default_font(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text("Hello World", 100, 100, 32, Color::RED);
    }

    ray_draw_test!(test_custom_font);
    fn test_custom_font(d: &mut RaylibDrawHandle<RaylibHandle>, assets: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text_ex(
            &assets.font,
            "Hello World",
            Vector2::new(100.0, 100.0),
            32.0,
            5.0,
            Color::RED,
        );
    }
}

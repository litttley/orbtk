extern crate orbtk;

use orbtk::*;

fn main() {
    let mut window = Window::new(Rect::new(0, 0, 400, 400), "OrbTK");

    window.widgets.push(
        Label::new(Rect::new(20, 20, 80, 16), "Test Label")
        .on_click(Box::new(|label: &mut Label, point: Point| {
            label.text = format!("{:?}", point);
            label.rect.width = label.text.chars().count() * 8;
        }))
    );

    window.widgets.push(
        ProgressBar::new(Rect::new(20, 60, 200, 16), 50)
        .on_click(Box::new(|progress_bar: &mut ProgressBar, point: Point| {
            progress_bar.value = point.x * 100 / progress_bar.rect.width as isize;
        }))
    );

    window.widgets.push(
        Button::new(Rect::new(20, 100, 100, 16), "Test Button")
        .on_click(Box::new(|button: &mut Button, point: Point| {
            button.text = format!("{:?}", point);
            button.rect.width = button.text.chars().count() * 8;
        }))
    );

    window.exec();
}

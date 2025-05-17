use imgui::*;

mod support;
mod flamegraph;

use flamegraph::{FlameGraphItem, FlameGraphWidget};

fn build_sample_data() -> Vec<FlameGraphItem> {
    vec![
        FlameGraphItem {
            label: "main".to_string(),
            value: 100.0,
            children: vec![
                FlameGraphItem {
                    label: "init".to_string(),
                    value: 30.0,
                    children: vec![],
                },
                FlameGraphItem {
                    label: "process".to_string(),
                    value: 50.0,
                    children: vec![
                        FlameGraphItem {
                            label: "parse".to_string(),
                            value: 20.0,
                            children: vec![],
                        },
                        FlameGraphItem {
                            label: "execute".to_string(),
                            value: 30.0,
                            children: vec![],
                        },
                    ],
                },
                FlameGraphItem {
                    label: "shutdown".to_string(),
                    value: 20.0,
                    children: vec![],
                },
            ],
        },
    ]
}

fn main() {
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    support::simple_init(file!(), move |_, ui| {
        let flame_data = build_sample_data();
        let mut flamegraph = FlameGraphWidget::new(&flame_data);
        ui.window("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(|| {
                flamegraph.render(&ui);

                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}

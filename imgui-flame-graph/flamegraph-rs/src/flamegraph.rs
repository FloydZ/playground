use imgui::{ImColor32, Ui};

#[derive(Clone)]
pub struct FlameGraphItem {
    pub label: String,
    pub value: f32,
    pub children: Vec<FlameGraphItem>,
}

pub struct FlameGraphWidget<'a> {
    pub root: &'a [FlameGraphItem],
    pub selected: Option<&'a FlameGraphItem>,
    pub value_total: f32,
    pub depth_max: usize,
}

impl<'a> FlameGraphWidget<'a> {
    pub fn new(root: &'a [FlameGraphItem]) -> Self {
        let value_total = Self::compute_total_value(root);
        let depth_max = Self::compute_max_depth(root, 0);
        FlameGraphWidget {
            root,
            selected: None,
            value_total,
            depth_max,
        }
    }

    fn compute_total_value(items: &[FlameGraphItem]) -> f32 {
        items.iter().map(|item| item.value).sum()
    }

    fn compute_max_depth(items: &[FlameGraphItem], depth: usize) -> usize {
        items
            .iter()
            .map(|item| {
                let child_depth = Self::compute_max_depth(&item.children, depth + 1);
                std::cmp::max(depth + 1, child_depth)
            })
            .max()
            .unwrap_or(depth)
    }

    pub fn render(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();
        // let p0 = ui.window_content_region_min();
        // let p1 = ui.window_content_region_max();
        let p0 = ui.cursor_screen_pos(); // top-left corner
        let region = ui.content_region_avail(); // size of the available content area
        let p1 = [p0[0] + region[0], p0[1] + region[1]]; 
        let mut pos = [p0[0], p0[1]];
        let available_width = p1[0] - p0[0];
        let row_height = 20.0;

        self.render_level(ui, &draw_list, self.root, &mut pos, 0, available_width, self.value_total, row_height);
    }

    fn render_level(
        &self,
        ui: &Ui,
        draw_list: &imgui::DrawListMut,
        items: &'a [FlameGraphItem],
        pos: &mut [f32; 2],
        depth: usize,
        width: f32,
        total: f32,
        row_height: f32,
    ) {
        let mut x = pos[0];
        let y = pos[1] + depth as f32 * row_height;

        for item in items {
            let item_width = width * (item.value / total);
            let rect_min = [x, y];
            let rect_max = [x + item_width, y + row_height];

            draw_list
                .add_rect(rect_min, rect_max, ImColor32::from_rgb(255, 100, 100))
                .filled(true)
                .build();

            draw_list
                .add_text([x + 3.0, y + 2.0],   ImColor32::WHITE, &item.label);

            // Optional: check for mouse hover/click using ImGui::IsMouseHoveringRect etc.

            // Recursively render children
            if !item.children.is_empty() {
                self.render_level(
                    ui,
                    draw_list,
                    &item.children,
                    pos,
                    depth + 1,
                    item_width,
                    item.value,
                    row_height,
                );
            }

            x += item_width;
        }
    }
}

use egui::{
    emath::{self, Rot2},
    epaint::RectShape,
    vec2, Color32, CursorIcon, Painter, Pos2, Rect, Response, RichText, Rounding, Sense, Shape,
    Stroke, Ui, Vec2,
};
pub trait View {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        image: egui::Image,
        mult_fact: &mut Option<(f32, f32)>,
        dim: Vec2,
        opt: PpOptions,
        save: bool,
        cut_clicked: bool,
    ) -> (
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Pos2, Color32, String)>>,
        Option<Vec<(Rect, Color32)>>,
        Option<Vec<(Pos2, f32, Color32)>>,
        Option<Response>,
    );
}

pub trait Demo {
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }
    fn name(&self) -> &'static str;
}
#[derive(Debug, Clone)]
pub enum PpOptions {
    Arrow,
    Circle,
    Square,
    Text,
    Painting,
    Cut,
}
pub struct Painting {
    last_type_added: Vec<PpOptions>,
    last_type_removed: Vec<PpOptions>,

    mult_factor: Option<(f32, f32)>,

    lines: Vec<(Vec<Pos2>, Color32)>,
    lines_color: egui::Color32,

    starting_point: Pos2,
    final_point: Pos2,
    arrows: Vec<(Pos2, Pos2, Color32)>,
    removed_arrows: Vec<(Pos2, Pos2, Color32)>,
    arrows_color: Color32,
    arrows_pixels: Vec<(Vec<Pos2>, Color32)>,

    circle_center: Pos2,
    radius: f32,
    circles: Vec<(Pos2, f32, Color32)>,
    removed_circles: Vec<(Pos2, f32, Color32)>,
    circles_color: Color32,

    square_starting_point: Pos2,
    square_ending_point: Pos2,
    squares_color: Color32,
    squares: Vec<(Rect, Color32)>,
    removed_squares: Vec<(Rect, Color32)>,
    shift_squares: Option<Pos2>,

    text_starting_position: Pos2,
    text_ending_position: Pos2,
    texts_color: Color32,
    texts: Vec<(String, Pos2, Pos2, Color32)>,
    removed_texts: Vec<(String, Pos2, Pos2, Color32)>,
    to_write_text: String,
    ready_to_write: bool,
    counter: i32,
    inizializzato: bool,
    entrato: bool,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            last_type_added: Vec::new(),
            last_type_removed: Vec::new(),

            mult_factor: None,
            lines: Default::default(),
            lines_color: Color32::from_rgba_unmultiplied(25, 200, 100, 255),

            starting_point: Pos2 { x: -1.0, y: -1.0 },
            final_point: Pos2 { x: -1.0, y: -1.0 },
            arrows: Vec::new(),
            removed_arrows: Vec::new(),
            arrows_color: Color32::from_rgba_unmultiplied(25, 200, 100, 255),
            arrows_pixels: Vec::new(),

            circle_center: Pos2 { x: -1.0, y: -1.0 },
            radius: -1.0,
            circles: Vec::new(),
            removed_circles: Vec::new(),
            circles_color: Color32::from_rgba_unmultiplied(25, 200, 100, 255),

            square_starting_point: Pos2 { x: -1.0, y: -1.0 },
            square_ending_point: Pos2 { x: -1.0, y: -1.0 },
            squares_color: Color32::from_rgba_unmultiplied(25, 200, 100, 255),
            squares: Vec::new(),
            removed_squares: Vec::new(),
            shift_squares: None,

            text_starting_position: Pos2 { x: -1.0, y: -1.0 },
            text_ending_position: Pos2 { x: -1.0, y: -1.0 },
            texts: Vec::new(),
            removed_texts: Vec::new(),
            texts_color: Color32::from_rgba_unmultiplied(25, 200, 100, 255),
            to_write_text: "Write something".to_string(),
            ready_to_write: false,
            counter: 0,
            inizializzato: false,
            entrato: false,
        }
    }
}

impl Painting {
    pub fn render_elements(&mut self, painter: Painter, to_screen: emath::RectTransform) {
        if !self.lines.is_empty() {
            let shapes = self
                .lines
                .iter()
                .filter(|line| line.0.len() >= 2)
                .map(|line| {
                    let points: Vec<Pos2> = line.0.iter().map(|p| to_screen * *p).collect();
                    egui::Shape::line(points, Stroke::new(0.5, line.1))
                });
            painter.extend(shapes);
        }
        if !self.arrows.is_empty() {
            for point in self.arrows.clone().into_iter() {
                painter.arrow(
                    point.0,
                    vec2(point.1.x - point.0.x, point.1.y - point.0.y),
                    Stroke::new(0.5, point.2),
                );
                let pixels = self
                    .calc_pixels_arrow(point.0, vec2(point.1.x - point.0.x, point.1.y - point.0.y));
                if !self.arrows_pixels.contains(&(pixels.clone(), point.2)) {
                    self.arrows_pixels.push((pixels, point.2));
                }
            }
        }

        if !self.circles.is_empty() {
            for point in self.circles.clone().into_iter() {
                painter.circle(
                    point.0,
                    point.1,
                    egui::Color32::TRANSPARENT,
                    Stroke::new(0.5, point.2),
                );
            }
        }

        if !self.squares.is_empty() || self.squares.len() == 0 {
            for point in self.squares.clone().into_iter() {
                painter.rect(
                    point.0,
                    0.0,
                    egui::Color32::TRANSPARENT,
                    Stroke::new(0.5, point.1),
                );
            }
        }

        if !self.texts.is_empty() {
            for point in self.texts.clone().into_iter() {
                painter.text(
                    point.1,
                    egui::Align2::LEFT_TOP,
                    point.0,
                    egui::FontId::monospace(15.0),
                    point.3,
                );
            }
        }
    }
    fn undo(&mut self) {
        match self.last_type_added.last().unwrap() {
            PpOptions::Arrow => {
                let rem = self.arrows.pop().unwrap();
                self.removed_arrows.push(rem);
                self.arrows_pixels.remove(self.arrows_pixels.len() - 1);
                self.last_type_removed.push(PpOptions::Arrow);
            }
            PpOptions::Circle => {
                let rem = self.circles.pop().unwrap();
                self.removed_circles.push(rem);
                //println!("{:?}", self.circles.len());
                self.last_type_removed.push(PpOptions::Circle);
            }
            PpOptions::Square => {
                let rem = self.squares.pop().unwrap();
                self.removed_squares.push(rem);
                self.last_type_removed.push(PpOptions::Square);
            }
            PpOptions::Text => {
                let rem = self.texts.pop().unwrap();
                self.removed_texts.push(rem);
                self.last_type_removed.push(PpOptions::Text);
            }
            _ => {}
        }
        self.last_type_added.pop();
    }
    fn redo(&mut self) {
        match self.last_type_removed.last().unwrap() {
            PpOptions::Arrow => {
                let rem = self.removed_arrows.pop().unwrap();
                self.arrows.push(rem);
                self.last_type_added.push(PpOptions::Arrow);
            }
            PpOptions::Circle => {
                let rem = self.removed_circles.pop().unwrap();
                self.circles.push(rem);
                self.last_type_added.push(PpOptions::Circle);
            }
            PpOptions::Square => {
                let rem = self.removed_squares.pop().unwrap();
                self.squares.push(rem);
                self.last_type_added.push(PpOptions::Square);
            }
            PpOptions::Text => {
                let rem = self.removed_texts.pop().unwrap();
                self.texts.push(rem);
                self.last_type_added.push(PpOptions::Text);
            }
            _ => {}
        }
        self.last_type_removed.pop();
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui, opt: PpOptions) -> egui::Response {
        let mut res = None;
        match opt {
            PpOptions::Painting => {
                if self.lines.last_mut() == None {
                    res = Some(
                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut self.lines_color);

                            ui.separator();
                            if ui.button("Clear Painting").clicked() {
                                self.lines.clear();
                            }
                        })
                        .response,
                    );
                    res.unwrap()
                } else {
                    let res = ui
                        .horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut self.lines.last_mut().unwrap().1);

                            ui.separator();
                            if ui.button("Clear Painting").clicked() {
                                self.lines.clear();
                            }
                        })
                        .response;
                    if !self.lines.is_empty() {
                        self.lines_color = self.lines.last_mut().unwrap().1;
                    }

                    res
                }
            }
            PpOptions::Arrow => {
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut self.arrows_color);
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add_enabled(true, egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    } else {
                        back_btn = Some(ui.add_enabled(false, egui::Button::new("↩")));
                    }
                    if self.last_type_removed.len() > 0 {
                        forward_btn = Some(ui.add_enabled(true, egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    } else {
                        forward_btn = Some(ui.add_enabled(false, egui::Button::new("↪")));
                    }
                })
                .response
            }
            PpOptions::Circle => {
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut self.circles_color);
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add_enabled(true, egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    } else {
                        back_btn = Some(ui.add_enabled(false, egui::Button::new("↩")));
                    }
                    if self.last_type_removed.len() > 0 {
                        forward_btn = Some(ui.add_enabled(true, egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    } else {
                        forward_btn = Some(ui.add_enabled(false, egui::Button::new("↪")));
                    }
                })
                .response
            }
            PpOptions::Square => {
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui: &mut Ui| {
                    ui.color_edit_button_srgba(&mut self.squares_color);
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add_enabled(true, egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    } else {
                        back_btn = Some(ui.add_enabled(false, egui::Button::new("↩")));
                    }
                    if self.last_type_removed.len() > 0 {
                        forward_btn = Some(ui.add_enabled(true, egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    } else {
                        forward_btn = Some(ui.add_enabled(false, egui::Button::new("↪")));
                    }
                })
                .response
            }
            PpOptions::Text => {
                let mut write_btn = None;
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui: &mut Ui| {
                    ui.color_edit_button_srgba(&mut self.texts_color);
                    ui.separator();
                    if ui
                        .add(egui::TextEdit::singleline(&mut self.to_write_text))
                        .clicked()
                        == false
                    {
                    } else {
                        if self.entrato == false {
                            self.entrato = true;
                            self.to_write_text = "".to_string();
                        }
                    }
                    ui.separator();
                    write_btn = Some(ui.add(egui::Button::new("Write!")));
                    if write_btn.unwrap().clicked()
                        && self.text_starting_position.x != -1.0
                        && self.text_starting_position.y != -1.0
                        && self.text_ending_position.x != -1.0
                        && self.text_ending_position.y != -1.0
                    {
                        self.entrato = true;
                        self.to_write_text = self.to_write_text.clone();
                        self.ready_to_write = true;
                    }
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add_enabled(true, egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    } else {
                        back_btn = Some(ui.add_enabled(false, egui::Button::new("↩")));
                    }
                    if self.last_type_removed.len() > 0 {
                        forward_btn = Some(ui.add_enabled(true, egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    } else {
                        forward_btn = Some(ui.add_enabled(false, egui::Button::new("↪")));
                    }
                })
                .response
            }
            PpOptions::Cut => ui.horizontal(|ui: &mut Ui| {}).response,

            _ => res.unwrap(),
        }
    }

    pub fn ui_content(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Vec<Pos2>, Color32)>>, Option<Response>) {
        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        image.paint_at(ui, response.rect);
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));
        self.shift_squares = Some(Pos2::new(
            response.rect.left_top().x,
            response.rect.left_top().y,
        ));
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }

        self.render_elements(painter.clone(), to_screen);

        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push((vec![], self.lines_color));
        }

        let current_line = &mut self.lines.last_mut().unwrap().0;
        let pointer_pos = response.interact_pointer_pos();
        if pointer_pos.is_none() == false && cut_clicked == false {
            let canvas_pos = from_screen * pointer_pos.unwrap();

            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push((vec![], self.lines_color));
            response.mark_changed();
        }

        self.render_elements(painter.clone(), to_screen);
        let mut ret = Vec::new();

        for l in self.lines.clone().into_iter() {
            let clr = l.1;
            let lns =
                l.0.into_iter()
                    .map(|f| from_screen.inverse().transform_pos(f));
            let mut retlns = Vec::new();

            for li in lns.clone().into_iter() {
                let ps = Pos2::new(
                    (li.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
                    (li.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
                );
                retlns.push(ps);
            }

            if retlns.len()>0{
             ret.push((retlns.clone(), clr));     
            }      
            
           
        }

        (Some(ret), Some(response))
    }

    pub fn ui_content_arrows(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Vec<Pos2>, Color32)>>, Option<Response>) {
        let (response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        image.paint_at(ui, response.rect);
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.starting_point.x == -1.0
                && self.starting_point.y == -1.0
            {
                self.starting_point = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_down()) && cut_clicked == false {
            let pos_dinamica = ui.input(|i| i.pointer.latest_pos());

            if pos_dinamica.is_none() == false
                && self.starting_point.x != -1.0
                && self.starting_point.y != -1.0
            {
                ui.painter().with_clip_rect(response.rect).arrow(
                    self.starting_point,
                    vec2(
                        pos_dinamica.unwrap().x - self.starting_point.x,
                        pos_dinamica.unwrap().y - self.starting_point.y,
                    ),
                    Stroke::new(0.5, self.arrows_color),
                )
            }
        }

        if ui.input(|i| i.pointer.any_released()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && self.final_point.x == -1.0
                && self.final_point.y == -1.0
                && self.starting_point.x != -1.0
                && self.starting_point.y != -1.0
            {
                self.final_point = pos.unwrap();
            }
        }

        if self.final_point.x != -1.0
            && self.final_point.y != -1.0
            && self.starting_point.x != -1.0
            && self.starting_point.y != -1.0
        {
            self.arrows
                .push((self.starting_point, self.final_point, self.arrows_color));
            self.starting_point = Pos2 { x: -1.0, y: -1.0 };
            self.final_point = Pos2 { x: -1.0, y: -1.0 };
            self.last_type_added.push(PpOptions::Arrow);
        }
        self.shift_squares = Some(Pos2::new(
            response.rect.left_top().x,
            response.rect.left_top().y,
        ));

        self.render_elements(painter.clone(), to_screen);

        (Some(self.arrows_pixels.clone()), Some(response))
    }

    pub fn ui_content_circles(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,

        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Pos2, f32, Color32)>>, Option<Response>) {
        let (response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));

        image.paint_at(ui, response.rect);

        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }

        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.latest_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.circle_center.x == -1.0
                && self.circle_center.y == -1.0
            {
                self.circle_center = ui.input(|i| i.pointer.interact_pos().unwrap());
            }
        }

        if ui.input(|i| i.pointer.any_down()) && cut_clicked == false {
            let pos_dinamica = ui.input(|i| i.pointer.latest_pos());

            if pos_dinamica.is_none() == false
                && self.circle_center.x != -1.0
                && self.circle_center.y != -1.0
            {
                let mut distanza = 0.0;

                if ((pos_dinamica.unwrap().x - self.circle_center.x) / 2.0).abs()
                    >= ((pos_dinamica.unwrap().y - self.circle_center.y) / 2.0).abs()
                {
                    distanza = ((pos_dinamica.unwrap().x - self.circle_center.x) / 2.0).abs();
                } else {
                    distanza = ((pos_dinamica.unwrap().y - self.circle_center.y) / 2.0).abs();
                }

                ui.painter().with_clip_rect(response.rect).circle(
                    self.circle_center,
                    distanza,
                    egui::Color32::TRANSPARENT,
                    Stroke::new(0.5, self.circles_color),
                );
            }
        }

        if ui.input(|i| i.pointer.any_released())
            && cut_clicked == false
            && self.circle_center.x != -1.0
            && self.circle_center.y != -1.0
            && self.radius == -1.0
        {
            let pos_finale = ui.input(|i| i.pointer.interact_pos());

            if ((pos_finale.unwrap().x - self.circle_center.x) / 2.0).abs()
                >= ((pos_finale.unwrap().y - self.circle_center.y) / 2.0).abs()
            {
                self.radius = ((pos_finale.unwrap().x - self.circle_center.x) / 2.0).abs();
            } else {
                self.radius = ((pos_finale.unwrap().y - self.circle_center.y) / 2.0).abs();
            }
        }

        if self.circle_center.x != -1.0 && self.circle_center.y != -1.0 && self.radius != -1.0 {
            self.circles
                .push((self.circle_center, self.radius, self.circles_color));
            self.circle_center = Pos2 { x: -1.0, y: -1.0 };
            self.radius = -1.0;
            self.last_type_added.push(PpOptions::Circle);
            self.shift_squares = Some(Pos2::new(
                response.rect.left_top().x,
                response.rect.left_top().y,
            ));
        }

        self.render_elements(painter.clone(), to_screen);
        let mut crcls = Vec::new();
       

        for c in self.circles.clone() {
            let center_x = (c.0.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0;
            let center_y = (c.0.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1;
            let new_center = Pos2::new(center_x, center_y);
            let new_radius = c.1 * self.mult_factor.unwrap().1;
            crcls.push((new_center, new_radius, c.2));
        }
        
        
        (Some(crcls.clone()), Some(response))
    }
    pub fn ui_content_squares(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,

        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Rect, Color32)>>, Option<Response>) {
        let (response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));

        image.paint_at(ui, response.rect);

        let mouse_pos = ui.input(|i| i.pointer.interact_pos());

        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked == false {
            let pos = response.interact_pointer_pos();
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.square_starting_point.x == -1.0
                && self.square_starting_point.y == -1.0
            {
                self.shift_squares = Some(Pos2::new(
                    response.rect.left_top().x,
                    response.rect.left_top().y,
                ));

                self.square_starting_point = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_down()) && cut_clicked == false {
            let pos_dinamica = ui.input(|i| i.pointer.latest_pos());

            if pos_dinamica.is_none() == false
                && self.square_starting_point.x != -1.0
                && self.square_starting_point.y != -1.0
            {
                ui.painter()
                    .with_clip_rect(response.rect)
                    .add(Shape::Rect(RectShape::new(
                        Rect::from_two_pos(self.square_starting_point, pos_dinamica.unwrap()),
                        Rounding::default(),
                        Color32::TRANSPARENT,
                        Stroke::new(0.5, self.squares_color),
                    )));
            }
        }

        if ui.input(|i| i.pointer.any_released()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.interact_pos());

            if pos.is_none() == false
                && self.square_ending_point.x == -1.0
                && self.square_ending_point.y == -1.0
                && self.square_starting_point.x != -1.0
                && self.square_starting_point.y != -1.0
            {
                self.square_ending_point = pos.unwrap();
            }
        }

        if self.square_starting_point.x != -1.0
            && self.square_starting_point.y != -1.0
            && self.square_ending_point.x != -1.0
            && self.square_ending_point.y != 1.0
        {
            let re =
                egui::Rect::from_points(&[self.square_starting_point, self.square_ending_point]);
            if re.area() > 0.0 && re.width() > 0.0 && re.height() > 0.0 {
                self.squares.push((re, self.squares_color));
                self.last_type_added.push(PpOptions::Square);
            }

            self.square_starting_point.x = -1.0;
            self.square_starting_point.y = -1.0;
            self.square_ending_point.x = -1.0;
            self.square_ending_point.y = -1.0;
        }

        self.render_elements(painter.clone(), to_screen);

        let mut sqrs = Vec::new();
  
            for s in self.squares.clone() {
                let min = Pos2::new(
                    (s.0.left_top().x - self.shift_squares.unwrap().x)
                        * self.mult_factor.unwrap().0,
                    (s.0.left_top().y - self.shift_squares.unwrap().y)
                        * self.mult_factor.unwrap().1,
                );
                let max = Pos2::new(
                    (s.0.right_bottom().x - self.shift_squares.unwrap().x)
                        * self.mult_factor.unwrap().0,
                    (s.0.right_bottom().y - self.shift_squares.unwrap().y)
                        * self.mult_factor.unwrap().1,
                );

                let r = egui::Rect::from_min_max(min, max);

                sqrs.push((r, s.1));
            }
        
        (Some(sqrs.clone()), Some(response))
    }

    pub fn ui_content_texts(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        mult_fact: &mut Option<(f32, f32)>,
        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Pos2, Color32, String)>>, Option<Response>) {
        let (response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);
        self.shift_squares = Some(Pos2::new(
            response.rect.left_top().x,
            response.rect.left_top().y,
        ));
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));

        *mult_fact = self.mult_factor;
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
        {
            ui.ctx().output_mut(|i| i.cursor_icon = CursorIcon::Text);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.text_starting_position.x == -1.0
                && self.text_starting_position.y == -1.0
            {
                self.text_starting_position = pos.unwrap();
                self.inizializzato = true;
            } else if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.inizializzato == true
            {
                self.text_starting_position = pos.unwrap();
            }
        }

        if self.ready_to_write == false
            && self.text_starting_position.x != -1.0
            && self.text_starting_position.y != -1.0
        {
            self.counter = self.counter + 1;
            if self.counter == 3000 {
                self.counter = 0;
            }

            if self.counter % 50 == 0 {
                ui.painter().add(egui::Shape::dashed_line(
                    &[
                        Pos2::new(
                            self.text_starting_position.x,
                            self.text_starting_position.y + 10.0,
                        ),
                        Pos2::new(
                            self.text_starting_position.x,
                            self.text_starting_position.y - 10.0,
                        ),
                    ],
                    Stroke::new(5.0, Color32::WHITE),
                    10.0,
                    0.0,
                ));
            } else {
                ui.painter().add(egui::Shape::dashed_line(
                    &[
                        Pos2::new(
                            self.text_starting_position.x,
                            self.text_starting_position.y + 10.0,
                        ),
                        Pos2::new(
                            self.text_starting_position.x,
                            self.text_starting_position.y - 10.0,
                        ),
                    ],
                    Stroke::new(5.0, Color32::BLACK),
                    10.0,
                    0.0,
                ));
            }
        }

        if ui.input(|i| i.pointer.any_released()) && cut_clicked == false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.text_ending_position.x == -1.0
                && self.text_ending_position.y == -1.0
                && self.text_starting_position.x != -1.0
                && self.text_starting_position.y != -1.0
            {
                self.text_ending_position = pos.unwrap();
            }
        }

        if self.text_starting_position.x != -1.0
            && self.text_starting_position.y != -1.0
            && self.text_ending_position.x != -1.0
            && self.text_ending_position.y != -1.0
        {
            self.render_elements(painter.clone(), to_screen);
            if self.ready_to_write {
                self.texts.push((
                    self.to_write_text.clone(),
                    self.text_starting_position,
                    self.text_ending_position,
                    self.texts_color,
                ));

                self.text_starting_position.x = -1.0;
                self.text_starting_position.y = -1.0;
                self.text_ending_position.x = -1.0;
                self.text_ending_position.y = -1.0;
                self.ready_to_write = false;
                self.inizializzato = false;
                self.last_type_added.push(PpOptions::Text);
            }
        }

        self.render_elements(painter.clone(), to_screen);
        let mut txt = Vec::new();
        for t in self.texts.clone() {
            let new_pos = Pos2::new(
                (t.1.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
                (t.1.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
            );
            txt.push((new_pos, t.3, t.0));
        }

        (Some(txt.clone()), Some(response))
    }

    pub fn ui_content_cut(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        mult_fact: &mut Option<(f32, f32)>,
        dim: Vec2,
        cut_clicked: bool,
    ) -> (Option<Vec<(Pos2, Color32, String)>>, Option<Response>) {
        let (response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);
        self.shift_squares = Some(Pos2::new(
            response.rect.left_top().x,
            response.rect.left_top().y,
        ));
        self.mult_factor = Some((
            image.size().unwrap().x as f32 / response.rect.width(),
            image.size().unwrap().y as f32 / response.rect.height(),
        ));

        *mult_fact = self.mult_factor;
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
            && ui.input(|i| i.pointer.any_down() == false)
        {
            ui.ctx().output_mut(|i| i.cursor_icon = CursorIcon::Grab);
        } else if mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y)
            && ui.input(|i| i.pointer.any_down() == true)
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Grabbing);
        }

        (None, Some(response))
    }

    pub fn calc_pixels_arrow(&mut self, origin: Pos2, vec: Vec2) -> Vec<Pos2> {
        let mut pixels = Vec::new();

        let new_vec = Vec2 {
            x: vec.x * self.mult_factor.unwrap().0,
            y: vec.y * self.mult_factor.unwrap().1,
        };
        let new_origin = Pos2::new(
            (origin.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
            (origin.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
        );
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let tip_length = new_vec.length() / 4.0;

        let tip = new_origin + new_vec;
        let dir = vec.normalized();

        pixels.push([new_origin, tip]);
        pixels.push([tip, tip - tip_length * (rot * dir)]);
        pixels.push([tip, tip - tip_length * (rot.inverse() * dir)]);

        pixels.concat()
    }
}

impl Demo for Painting {
    fn name(&self) -> &'static str {
        "🖊 Painting"
    }
}

impl View for Painting {
    fn ui(
        &mut self,
        ui: &mut Ui,
        image: egui::widgets::Image,
        mult_fact: &mut Option<(f32, f32)>,
        dim: Vec2,
        opt: PpOptions,
        save: bool,
        cut_clicked: bool,
    ) -> (
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Pos2, Color32, String)>>,
        Option<Vec<(Rect, Color32)>>,
        Option<Vec<(Pos2, f32, Color32)>>,
        Option<Response>,
    ) {
        let mut pix = None;
        let mut arr = None;
        let mut txt = None;
        let mut sqrs = None;
        let mut crcls = None;
        let mut response = None;

        if save {
            self.last_type_added.clear();
            self.lines.clear();
            self.arrows.clear();
            self.arrows_pixels.clear();
            self.circles.clear();
            self.squares.clear();
            self.texts.clear();
        }

        match opt {
            PpOptions::Painting => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("Paint with your mouse/touch! If you want to clear all the painting, click the button Clear Painting").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    if image.size().unwrap()[0] >= 1000.0 && image.size().unwrap()[1] <= 500.0 {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                                (pix, response) = self.ui_content(ui, image, dim, cut_clicked);
                            });
                        });
                    } else {
                        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                            (pix, response) = self.ui_content(ui, image, dim, cut_clicked);
                        });
                    }
                });
            }
            PpOptions::Arrow => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("Paint an arrow with your mouse/touch! Press the left button of your mouse wherever you want, as a starting point, and release it when you want to finish drawing the arrow ").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (arr, response) = self.ui_content_arrows(ui, image, dim, cut_clicked);
                    });
                });
            }
            PpOptions::Circle => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("Paint a circle with your mouse/touch! Press the left button of your mouse wherever you want, to identify the circle's center, and release it when you want to finish drawing the circle").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (crcls, response) = self.ui_content_circles(ui, image, dim, cut_clicked);
                    });
                });
            }
            PpOptions::Square => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("Paint a square with your mouse/touch! Press the left button of your mouse wherever you want, to identify the rectangle's top-left corner, and release it when you want to set the right-bottom corner").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (sqrs, response) = self.ui_content_squares(ui, image, dim, cut_clicked);
                    });
                });
            }
            PpOptions::Text => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("First, click were you want to write and type your text in the bar above! When you finish writing, press the button Write! to insert your text on the image below").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (txt, response) =
                            self.ui_content_texts(ui, image, mult_fact, dim, cut_clicked);
                    });
                });
            }

            PpOptions::Cut => {
                self.ui_control(ui, opt);
                ui.label(RichText::new("Restrict the grabbed image however you want and when you identify the right area to cut, press the button Finish Your Cut ").color(Color32::WHITE));
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (txt, response) =
                            self.ui_content_cut(ui, image, mult_fact, dim, cut_clicked);
                    });
                });
            }
        }
        if self.last_type_removed.last().is_some() {
            match self.last_type_removed.last().unwrap() {
                PpOptions::Arrow => {
                    arr = Some(self.arrows_pixels.clone());
                }
                PpOptions::Circle => {
                    let mut circls = Vec::new();
                    for c in self.circles.clone() {
                        let center_x =
                            (c.0.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0;
                        let center_y =
                            (c.0.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1;
                        let new_center = Pos2::new(center_x, center_y);
                        let new_radius = c.1 * self.mult_factor.unwrap().1;
                        circls.push((new_center, new_radius, c.2));
                    }
                    crcls = Some(circls.clone());
                }
                PpOptions::Square => {
                    let mut sqars = Vec::new();
                    for s in self.squares.clone() {
                        let min = Pos2::new(
                            (s.0.left_top().x - self.shift_squares.unwrap().x)
                                * self.mult_factor.unwrap().0,
                            (s.0.left_top().y - self.shift_squares.unwrap().y)
                                * self.mult_factor.unwrap().1,
                        );
                        let max = Pos2::new(
                            (s.0.right_bottom().x - self.shift_squares.unwrap().x)
                                * self.mult_factor.unwrap().0,
                            (s.0.right_bottom().y - self.shift_squares.unwrap().y)
                                * self.mult_factor.unwrap().1,
                        );

                        let r = egui::Rect::from_min_max(min, max);

                        sqars.push((r, s.1));
                    }
                    sqrs = Some(sqars.clone());
                }
                PpOptions::Text => {
                    let mut tx = Vec::new();
                    for t in self.texts.clone() {
                        let new_pos = Pos2::new(
                            (t.1.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
                            (t.1.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
                        );
                        tx.push((new_pos, t.3, t.0));
                    }
                    txt = Some(tx.clone());
                }
                _ => {}
            }
        }
        *mult_fact = self.mult_factor;
        (pix, arr, txt, sqrs, crcls, response)
    }
}

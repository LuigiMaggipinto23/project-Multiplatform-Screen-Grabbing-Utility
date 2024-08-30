use std::f32::consts::PI;

use egui::{
    emath::{self, Rot2},
    vec2, Color32, CursorIcon, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2, Response,
};

/// Something to view in the demo windows
pub trait View {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        image: egui::Image,
        mult_fact: &mut Option<(f32, f32)>,
        dim: Vec2,
        opt: PpOptions,
        save:bool,
        cut_clicked:bool,
    ) -> (
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Pos2, Color32, String)>>,
        Option<Vec<(Rect, Stroke)>>,
        Option<Vec<(Pos2, f32, Stroke)>>,
        Option<Response>
    );
}

/// Something to view
pub trait Demo {
    /// Is the demo enabled for this integraton?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    // Show windows, etc
    /*fn show(&mut self, ctx: &egui::Context, open: &mut bool);*/
}
#[derive(Debug, Clone)]
pub enum PpOptions {
    Arrow,
    Circle,
    Square,
    Text,
    Painting,
}
pub struct Painting {
    last_type_added: Vec<PpOptions>,
    last_type_removed:Vec<PpOptions>,
    

    mult_factor: Option<(f32, f32)>,
    /// in 0-1 normalized coordinates
    lines: Vec<(Vec<Pos2>, Stroke)>,
    lines_stroke: Stroke,

    starting_point: Pos2,
    final_point: Pos2,
    arrows: Vec<(Pos2, Pos2, Stroke)>,
    removed_arrows:Vec<(Pos2, Pos2, Stroke)>,
    arrows_stroke: Stroke,
    arrows_pixels: Vec<(Vec<Pos2>, Color32)>,

    circle_center: Pos2,
    radius: f32,
    circles: Vec<(Pos2, f32, Stroke)>,
    removed_circles: Vec<(Pos2, f32, Stroke)>,
    circles_stroke: Stroke,

    square_starting_point: Pos2,
    square_ending_point: Pos2,
    squares_stroke: Stroke,
    squares: Vec<(Rect, Stroke)>,
    removed_squares:Vec<(Rect, Stroke)>,
    shift_squares: Option<Pos2>,

    text_starting_position: Pos2,
    text_ending_position: Pos2,
    texts_stroke: Stroke,
    texts: Vec<(String, Pos2, Pos2, Stroke)>,
    removed_texts: Vec<(String, Pos2, Pos2, Stroke)>,
    to_write_text: String,
    ready_to_write: bool,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            last_type_added: Vec::new(),
            last_type_removed:Vec::new(),  
                    

            mult_factor: None,
            lines: Default::default(),
            lines_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),

            starting_point: Pos2 { x: -1.0, y: -1.0 },
            final_point: Pos2 { x: -1.0, y: -1.0 },
            arrows: Vec::new(),
            removed_arrows:Vec::new(),
            arrows_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            arrows_pixels: Vec::new(),

            circle_center: Pos2 { x: -1.0, y: -1.0 },
            radius: -1.0,
            circles: Vec::new(),
            removed_circles:Vec::new(),
            circles_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),


            square_starting_point: Pos2 { x: -1.0, y: -1.0 },
            square_ending_point: Pos2 { x: -1.0, y: -1.0 },
            squares_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            squares: Vec::new(),
            removed_squares:Vec::new(),
            shift_squares: None,

            text_starting_position: Pos2 { x: -1.0, y: -1.0 },
            text_ending_position: Pos2 { x: -1.0, y: -1.0 },
            texts: Vec::new(),
            removed_texts:Vec::new(),
            texts_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            to_write_text: "Write something".to_string(),
            ready_to_write: false,
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
                    // let (vecio, stroke) = line.clone();
                    // if !self.linea_scritta {
                    //     for posizione in vecio {
                    //         self.lines_pixels.push(posizione);
                    //     }
                    //     self.linea_scritta = true;
                    // }

                    egui::Shape::line(points, line.1)
                });
            painter.extend(shapes);
        }
        if !self.arrows.is_empty() {
            for point in self.arrows.clone().into_iter() {
                painter.arrow(
                    point.0,
                    vec2(point.1.x - point.0.x, point.1.y - point.0.y),
                    point.2,
                );
                let pixels = self
                    .calc_pixels_arrow(point.0, vec2(point.1.x - point.0.x, point.1.y - point.0.y));
                if !self.arrows_pixels.contains(&(pixels.clone(), point.2.color)){
                    self.arrows_pixels.push((pixels, point.2.color));
                }
                
            }
        }

        if !self.circles.is_empty() {
            for point in self.circles.clone().into_iter() {
                painter.circle(point.0, point.1, egui::Color32::TRANSPARENT, point.2);                
            }
        }

        if !self.squares.is_empty() || self.squares.len() == 0 {
            for point in self.squares.clone().into_iter() {
                painter.rect(point.0, 0.0, egui::Color32::TRANSPARENT, point.1);
            }
        }

        if !self.texts.is_empty() {
            for point in self.texts.clone().into_iter() {
                painter.text(
                    point.1,
                    egui::Align2::LEFT_TOP,
                    point.0,
                    egui::FontId::monospace(15.0),
                    point.3.color,
                );
            }
        }
    }
    fn undo(&mut self) {
        match self.last_type_added.last().unwrap() {
            PpOptions::Arrow => {
                let rem=self.arrows.pop().unwrap();
                self.removed_arrows.push(rem);
                //self.arrows.remove(self.arrows.len() - 1);              
                self.arrows_pixels.remove(self.arrows_pixels.len()-1);    
                self.last_type_removed.push(PpOptions::Arrow);            
            }
            PpOptions::Circle => {
                let rem=self.circles.pop().unwrap();
                self.removed_circles.push(rem);
                //self.circles.remove(self.circles.len() - 1);
                self.last_type_removed.push(PpOptions::Circle);  
            }
            PpOptions::Square => {
                let rem=self.squares.pop().unwrap();
                self.removed_squares.push(rem);
                //self.squares.remove(self.squares.len() - 1);
                self.last_type_removed.push(PpOptions::Square);  
            }
            PpOptions::Text => {
                let rem=self.texts.pop().unwrap();
                self.removed_texts.push(rem);
               // self.texts.remove(self.texts.len() - 1);
               self.last_type_removed.push(PpOptions::Text);  
            }
            _ => {}
        }
        self.last_type_added.pop();
    }
    fn redo(&mut self) {
        match self.last_type_removed.last().unwrap() {
            PpOptions::Arrow => {
                let rem=self.removed_arrows.pop().unwrap();
                self.arrows.push(rem);        
                self.last_type_added.push(PpOptions::Arrow);      
                //self.arrows_pixels.remove(self.arrows_pixels.len()-1);                
            }
            PpOptions::Circle => {
                let rem=self.removed_circles.pop().unwrap();
                self.circles.push(rem);
                self.last_type_added.push(PpOptions::Circle); 
            }
            PpOptions::Square => {
                let rem=self.removed_squares.pop().unwrap();
                self.squares.push(rem);
                self.last_type_added.push(PpOptions::Square); 
            }
            PpOptions::Text => {
                let rem=self.removed_texts.pop().unwrap();
                self.texts.push(rem);
                self.last_type_added.push(PpOptions::Text); 
            }
            _ => {}
        }
        self.last_type_removed.pop();
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui, opt: PpOptions) -> egui::Response {
        // println!("In ui_control");
        let mut res = None;
        match opt {
            PpOptions::Painting => {
                if self.lines.last_mut() == None {
                    res = Some(
                        ui.horizontal(|ui| {
                            egui::stroke_ui(ui, &mut self.lines_stroke, "Stroke");
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
                            egui::stroke_ui(ui, &mut self.lines.last_mut().unwrap().1, "Stroke");
                            ui.separator();
                            if ui.button("Clear Painting").clicked() {
                                self.lines.clear();
                            }
                        })
                        .response;
                    if !self.lines.is_empty() {
                        self.lines_stroke = self.lines.last_mut().unwrap().1;
                    }

                    res
                }
            }
            PpOptions::Arrow => {
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui| {
                    egui::stroke_ui(ui, &mut self.arrows_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    if self.last_type_removed.len()>0{
                        forward_btn = Some(ui.add(egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    }
                })
                .response
            }
            PpOptions::Circle => {
                //println!("In ui_control circles");
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui| {
                    egui::stroke_ui(ui, &mut self.circles_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    if self.last_type_removed.len()>0{
                        forward_btn = Some(ui.add(egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    }
                })
                .response
            }
            PpOptions::Square => {
                //println!("In ui_control squares");
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui: &mut Ui| {
                    egui::stroke_ui(ui, &mut self.squares_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    if self.last_type_removed.len()>0{
                        forward_btn = Some(ui.add(egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    }
                })
                .response
            }
            PpOptions::Text => {
                // println!("In ui_control texts");
                let mut write_btn = None;
                let mut back_btn = None;
                let mut forward_btn = None;
                ui.horizontal(|ui: &mut Ui| {
                    egui::stroke_ui(ui, &mut self.texts_stroke, "Stroke");
                    ui.separator();
                    ui.add(egui::TextEdit::singleline(&mut self.to_write_text));
                    ui.separator();
                    write_btn = Some(ui.add(egui::Button::new("Write!")));
                    if write_btn.unwrap().clicked()
                        && self.text_starting_position.x != -1.0
                        && self.text_starting_position.y != -1.0
                        && self.text_ending_position.x != -1.0
                        && self.text_ending_position.y != -1.0
                    {
                        self.to_write_text = self.to_write_text.clone();
                        self.ready_to_write = true;
                    }
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("↩")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    if self.last_type_removed.len()>0{
                        forward_btn = Some(ui.add(egui::Button::new("↪")));
                        if forward_btn.unwrap().clicked() {
                            self.redo();
                        }
                    }
                })
                .response
            }
            _ => res.unwrap(),
        }
    }

    pub fn ui_content(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
        cut_clicked:bool
    ) -> (Option<Vec<(Vec<Pos2>, Color32)>>,Option<Response>){
        //println!("In ui_content");

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
        if (mouse_pos.is_none() == false
            
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }

        self.render_elements(painter.clone(), to_screen);

        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push((vec![], self.lines_stroke));
        }

        let mut current_line = &mut self.lines.last_mut().unwrap().0;
        let pointer_pos= response.interact_pointer_pos();
        if pointer_pos.is_none()==false && cut_clicked==false {
            let canvas_pos = from_screen * pointer_pos.unwrap();

            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push((vec![], self.lines_stroke));
            response.mark_changed();
        }

        self.render_elements(painter.clone(), to_screen);
        let mut ret = Vec::new();

        for l in self.lines.clone().into_iter() {
            let clr = l.1.color;
            let mut lns =
                l.0.into_iter()
                    .map(|f| from_screen.inverse().transform_pos(f));
            let mut retlns = Vec::new();
            for mut li in lns.into_iter() {
                let ps = Pos2::new(
                    (li.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
                    (li.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
                );
                retlns.push(ps);
            }
            if retlns.len()>0{
                ret.push((retlns, clr));
            }
            
        }
        

        (Some(ret),Some(response))
    }

    pub fn ui_content_arrows(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
        cut_clicked:bool
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
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked==false{
           
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
            
                && response.rect.contains(pos.unwrap())
                && self.starting_point.x == -1.0
                && self.starting_point.y == -1.0
            {
                self.starting_point = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_released()) && cut_clicked==false {
           
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
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
                .push((self.starting_point, self.final_point, self.arrows_stroke));
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
        cut_clicked:bool
    ) -> (Option<Vec<(Pos2, f32, Stroke)>>, Option<Response>) {
       

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

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
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked==false{
            let pos = ui.input(|i| i.pointer.latest_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.circle_center.x == -1.0
                && self.circle_center.y == -1.0
            {
                self.circle_center = ui.input(|i| i.pointer.interact_pos().unwrap());
            }
        }

        if ui.input(|i| i.pointer.any_released())
            && cut_clicked==false
            && self.circle_center.x != -1.0
            && self.circle_center.y != -1.0
            && self.radius == -1.0
        {
            self.radius = ui.input(|i| i.pointer.interact_pos().unwrap()).x - self.circle_center.x;
            self.radius = self.radius.abs();
        }

        if self.circle_center.x != -1.0 && self.circle_center.y != -1.0 && self.radius != -1.0 {
            self.circles
                .push((self.circle_center, self.radius, self.circles_stroke));
            self.circle_center = Pos2 { x: -1.0, y: -1.0 };
            self.radius = -1.0;
            self.last_type_added.push(PpOptions::Circle);
            self.shift_squares = Some(Pos2::new(
                response.rect.left_top().x,
                response.rect.left_top().y,
            ));
        }
        self.render_elements(painter.clone(), to_screen);
        let mut crcls=Vec::new();
        for c in self.circles.clone(){
            let center_x = (c.0.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0;
            let center_y = (c.0.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1;
            let new_center=Pos2::new(center_x, center_y);
            let new_radius=c.1*self.mult_factor.unwrap().1;
            crcls.push((new_center, new_radius, c.2));
        }
        (Some(crcls.clone()), Some(response))
    }

    pub fn ui_content_squares(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,

        dim: Vec2,
        cut_clicked:bool
    ) -> (Option<Vec<(Rect, Stroke)>>, Option<Response>) {
        //println!("In ui_content squares");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

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

        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked==false {
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

        if ui.input(|i| i.pointer.any_released())  && cut_clicked==false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
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

            self.squares.push((re, self.squares_stroke));
            self.square_starting_point.x = -1.0;
            self.square_starting_point.y = -1.0;
            self.square_ending_point.x = -1.0;
            self.square_ending_point.y = -1.0;
            self.last_type_added.push(PpOptions::Square);
        }

        self.render_elements(painter.clone(), to_screen);

        //Some(self.squares_pixels.clone())
        let mut sqrs = Vec::new();
        for s in self.squares.clone() {
            let min = Pos2::new(
                (s.0.left_top().x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0,
                (s.0.left_top().y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1,
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
        cut_clicked:bool
    ) -> (Option<Vec<(Pos2, Color32, String)>>, Option<Response>) {
        // println!("In ui_content texts");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

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
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx().output_mut(|i| i.cursor_icon = CursorIcon::Text);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) && cut_clicked==false {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.text_starting_position.x == -1.0
                && self.text_starting_position.y == -1.0
            {
                self.text_starting_position = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_released())  && cut_clicked==false {
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
                    self.texts_stroke,
                ));

                self.text_starting_position.x = -1.0;
                self.text_starting_position.y = -1.0;
                self.text_ending_position.x = -1.0;
                self.text_ending_position.y = -1.0;
                self.ready_to_write = false;
                self.last_type_added.push(PpOptions::Text);
            }
        }

        self.render_elements(painter.clone(), to_screen);
        let mut txt=Vec::new();
        for t in self.texts.clone(){
            let new_pos = Pos2::new(
                (t.1.x - self.shift_squares.unwrap().x)
                    * self.mult_factor.unwrap().0,
                (t.1.y - self.shift_squares.unwrap().y)
                    * self.mult_factor.unwrap().1,
            );
            txt.push((new_pos,  t.3.color, t.0 ));
        }
        
        (Some(txt.clone()), Some(response))
    }

    pub fn calc_pixels_arrow(&mut self, origin: Pos2, vec: Vec2) -> Vec<Pos2> {
        let mut pixels = Vec::new();

        let new_vec = Vec2 {
            x: vec.x * self.mult_factor.unwrap().0,
            y: vec.y * self.mult_factor.unwrap().1,
        };
        let new_origin = Pos2::new(
            ((origin.x - self.shift_squares.unwrap().x) * self.mult_factor.unwrap().0),
            ((origin.y - self.shift_squares.unwrap().y) * self.mult_factor.unwrap().1),
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
        save:bool,
        cut_clicked:bool,
    ) -> (
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Vec<Pos2>, Color32)>>,
        Option<Vec<(Pos2, Color32, String)>>,
        Option<Vec<(Rect, Stroke)>>,
        Option<Vec<(Pos2, f32, Stroke)>>,
        Option<Response>
    ) {
        let mut pix = None;
        let mut arr=None;
        let mut txt = None;
        let mut sqrs = None;
        let mut crcls=None;
        let mut response=None;

        if save{
            self.last_type_added.clear();
            self.lines.clear();
            self.arrows.clear();
            self.arrows_pixels.clear();
            self.circles.clear();
            self.squares.clear();
            self.texts.clear();
        }
        // if cut_clicked{
        //     self.arrows.clear();
        //     self.circles.clear();
        //     self.squares.clear();
        //     self.texts.clear();
        //     self.lines.clear();
        // }

       
        match opt {
            PpOptions::Painting => {
                self.ui_control(ui, opt);
                ui.label("Paint with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    if  image.size().unwrap()[0] >= 1000.0 && image.size().unwrap()[1] <= 500.0 {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui|{
                            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                                (pix,response) = self.ui_content(ui, image, dim, cut_clicked);
                               
                            });
                        });
                    }else{
                        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                            (pix,response) = self.ui_content(ui, image, dim, cut_clicked);
                           
                        });
                    }
                    
                });
            }
            PpOptions::Arrow => {
                self.ui_control(ui, opt);
                ui.label("Paint an arrow with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (arr,response) = self.ui_content_arrows(ui, image, dim, cut_clicked);
                        
                    });
                });
            }
            PpOptions::Circle => {
                self.ui_control(ui, opt);
                ui.label("Paint a circle with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (crcls,response) = self.ui_content_circles(ui, image, dim, cut_clicked);
                       
                    });
                });
            }
            PpOptions::Square => {
                self.ui_control(ui, opt);
                ui.label("Paint a square with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (sqrs,response) = self.ui_content_squares(ui, image, dim, cut_clicked);
                        
                    });
                });
            }
            PpOptions::Text => {
                self.ui_control(ui, opt);
                ui.label("First, click were you want to write and then write something!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        (txt,response) = self.ui_content_texts(ui, image, mult_fact, dim, cut_clicked);
                        
                    });
                });
            }
        }
        
        *mult_fact=self.mult_factor;
        (pix, arr, txt, sqrs, crcls,response)
    }
}
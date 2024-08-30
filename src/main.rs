// mod post_processing;
// use crate::post_processing::PpOptions;
// use crate::post_processing::View;
mod pp_no_stroke;
mod hotkeys;
use crate::pp_no_stroke::PpOptions;
use crate::pp_no_stroke::View;

use egui::CursorIcon;

use hotkeys::CustomizeHotkey;
use hotkeys::Hotkeys;
use image::EncodableLayout;
use rfd::FileDialog;
use arboard::Clipboard;
use egui_notify::Toasts;
mod functions;
use eframe::{
    egui::{self, Color32, RichText},
    Frame,
};
use egui::{epaint::RectShape, Pos2, Rect, Rounding, Shape, Stroke, TextureHandle, Vec2};

use screenshots::Screen;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

use global_hotkey::{
    hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager, HotKeyState,
};
use keyboard_types::{Code};

#[derive(PartialEq, Debug)]
enum ModeOptions {
    Rectangle,
    FullScreen,
}
#[derive(PartialEq, Debug)]
enum Shapes {
    None,
    Arrow,
    Circle,
    Square,
}

#[derive(PartialEq, Debug)]
enum TimerOptions {
    NoTimer,
    ThreeSeconds,
    FiveSeconds,
    TenSeconds,
}

#[derive(PartialEq, Debug)]
enum LoadingState {
    Loaded,
    NotLoaded,
}

#[derive(PartialEq, Debug)]
enum ImageFormat {
    Jpg,
    Png,
    Gif,
}


fn main() -> Result<(), eframe::Error> {
    let mut filepath = Some(PathBuf::new());

    let current_os = if cfg!(unix) {
        let _ = std::fs::create_dir("./screenshot");
        filepath = Some(PathBuf::from("./screenshot"));
        "unix"
    } else if cfg!(windows) {
        let _ = std::fs::create_dir(".//screenshot");
        filepath = Some(PathBuf::from(".//screenshot"));
        "windows"
    } else {
        "unknown"
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(680.0, 480.0)),
        transparent: true,
        follow_system_theme:false,
        default_theme:eframe::Theme::Dark,
        ..Default::default()
    };

    let manager = GlobalHotKeyManager::new().unwrap();
    let shortcuts = Hotkeys::new();
    manager.register_all(shortcuts.get_hotkeys().as_slice()).unwrap();
    //let p = post_processing::Painting::default();
    let p=pp_no_stroke::Painting::default();

    let openfw = GlobalHotKeyEvent::receiver();

    eframe::run_native(
        "Screen Grabbing Utility",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(FirstWindow {
                toasts:Some(Toasts::default()),
                show_toast:false,
                number_of_screens:None,
                screen_to_show: None,
                frame_initial_pos:None,
                image_name: None,
                image_format: Some(ImageFormat::Jpg),
                image_format_string: "jpg".to_string(),
                pp_option: None,
                current_os: current_os.to_string(),
                multiplication_factor: None,
                screen_size: None,
                loading_state: LoadingState::NotLoaded,
                image: None,
                image_texture: None,
                image_buffer: None,
                filepath: filepath,
                selected_mode: ModeOptions::Rectangle,
                selected_mode_string: "Rectangle".to_string(),
                selected_timer: TimerOptions::NoTimer,
                selected_timer_string: "No timer".to_string(),
                selected_timer_numeric: 0 as u64,
                selected_shape: Shapes::None,
                selected_shape_string: "Select a shape!".to_string(),
                selected_window: 1,
                mouse_pos: Option::Some(egui::pos2(-1.0, -1.0)),
                mouse_pos_f: Option::Some(egui::pos2(-1.0, -1.0)),
                rect_pos: egui::pos2(0.0, 0.0),
                rect_pos_f: egui::pos2(0.0, 0.0),
                open_fw: openfw.clone(),
                screenshots_taken: None,
                painting: p,
                width: 0.0,
                height: 0.0,
                mult_factor: None,
                cut_clicked: false,
                cropped:false,
                circle_pixels: Vec::new(),
                square_pixels: Vec::new(),
                arrow_pixels: Vec::new(),
                text_pixels: Vec::new(),
                line_pixels: Vec::new(),
                save:false,
                ready_to_save: false,
                ready_to_save_with_name: false,
                ready_to_copy: false,
                ready_to_crop: false,
                customizing_hotkey: usize::MAX,
                new_hotkey: CustomizeHotkey::default(),
                to_cut_rect:None,
                shrink_fact:None,
                shortcuts: shortcuts,
                manager: manager,
                ready_to_cut:None,
                is_pointer_on_cut_window: false,
                set_Wh_window:true,
                dim_bool: false,
                first_time:true,
            })
        }),
    )
}

struct FirstWindow {
    toasts:Option<Toasts>,
    show_toast:bool,
    number_of_screens:Option<usize>,
    screen_to_show: Option<u32>,
    frame_initial_pos:Option<Pos2>,
    image_name: Option<String>,
    image_format: Option<ImageFormat>,
    image_format_string: String,
    pp_option: Option<PpOptions>,
    current_os: String,
    multiplication_factor: Option<f32>,
    screen_size: Option<Vec2>,
    loading_state: LoadingState,
    image: Option<TextureHandle>,
    image_texture: Option<egui::ColorImage>,
    image_buffer: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    filepath: Option<PathBuf>,
    selected_mode: ModeOptions,
    selected_mode_string: String,
    selected_timer: TimerOptions,
    selected_timer_string: String,
    selected_timer_numeric: u64,
    selected_shape: Shapes,
    selected_shape_string: String,
    selected_window: usize,
    mouse_pos: Option<Pos2>,
    mouse_pos_f: Option<Pos2>,
    rect_pos: Pos2,
    rect_pos_f: Pos2,
    open_fw: GlobalHotKeyEventReceiver,
    screenshots_taken: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    //painting: post_processing::Painting,
    painting: pp_no_stroke::Painting,
    width: f32,
    height: f32,
    mult_factor: Option<(f32, f32)>,
    cut_clicked: bool,
    cropped:bool,
    circle_pixels: Vec<(Pos2, f32, Color32)>,
    square_pixels: Vec<(Rect, Color32)>,
    arrow_pixels: Vec<(Vec<Pos2>, Color32)>,
    text_pixels: Vec<(Pos2, Color32, String)>,
    line_pixels: Vec<(Vec<Pos2>, Color32)>,
    save:bool,
    ready_to_save:bool,
    ready_to_save_with_name: bool,
    ready_to_copy: bool,
    ready_to_crop: bool,
    customizing_hotkey:usize,
    new_hotkey: CustomizeHotkey,
    to_cut_rect:Option<(Pos2, Pos2)>,
    shrink_fact:Option<f32>,
    shortcuts: Hotkeys,
    manager: GlobalHotKeyManager,
    ready_to_cut:Option<bool>,
    is_pointer_on_cut_window: bool,
    set_Wh_window:bool,
    dim_bool:bool,
    first_time:bool,
    
}

impl eframe::App for FirstWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {   

        ctx.request_repaint(); 
            
        let screens=Screen::all().unwrap();

        if self.screen_to_show.is_none(){
            self.screen_to_show=Some(screens[0].display_info.id);
            self.screen_size=Some(Vec2::new(screens[0].display_info.width as f32, screens[0].display_info.height as f32));
            self.frame_initial_pos=Some(Pos2::new(screens[0].display_info.x as f32, screens[0].display_info.y as f32));
        }
        self.number_of_screens=Some(screens.len());
        if self.multiplication_factor.is_none() {
            self.multiplication_factor = frame.info().native_pixels_per_point;
        }
    



         

        if self.selected_window == 1 {
            self.hotkey_listener();
            self.mouse_pos=Some(Pos2::new(-1.0, -1.0));
            self.mouse_pos_f=Some(Pos2::new(-1.0, -1.0));
            self.rect_pos =  egui::pos2(0.0, 0.0);
            self.rect_pos_f =  egui::pos2(0.0, 0.0);
            frame.set_decorations(true); 
            frame.set_window_size(egui::vec2(680.0, 480.0)); 

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0); // da modificare
                    
                    if ui
                        .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                        .on_hover_text(self.shortcuts.get_hotkey_strings_formatted(1))
                        .clicked()
                    {
                        std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
                        self.selected_window = 2;
                    }

                    egui::ComboBox::from_id_source("mode_Combobox")
                        .width(200.0)
                        .selected_text(
                            RichText::new(format!("{}", self.selected_mode_string)).size(30.0),
                        )
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.selected_mode,
                                    ModeOptions::Rectangle,
                                    RichText::new("Rectangle").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_mode_string = "Rectangle".to_string();
                            }
                            if ui
                                .selectable_value(
                                    &mut self.selected_mode,
                                    ModeOptions::FullScreen,
                                    RichText::new("Full Screen").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_mode_string = "Full Screen".to_string();
                            };
                        });

                    egui::ComboBox::from_id_source("timer_Combobox")
                        .width(200.0)
                        .selected_text(
                            RichText::new(format!("{}", self.selected_timer_string)).size(30.0),
                        )
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.selected_timer,
                                    TimerOptions::NoTimer,
                                    RichText::new("No Timer").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_timer_string = "No Timer".to_string();
                                self.selected_timer_numeric=0;
                            };

                            if ui
                                .selectable_value(
                                    &mut self.selected_timer,
                                    TimerOptions::ThreeSeconds,
                                    RichText::new("3 Seconds").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_timer_string = "3 Seconds".to_string();
                                self.selected_timer_numeric = 3;
                            };
                            if ui
                                .selectable_value(
                                    &mut self.selected_timer,
                                    TimerOptions::FiveSeconds,
                                    RichText::new("5 Seconds").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_timer_string = "5 Seconds".to_string();
                                self.selected_timer_numeric = 5;
                            };
                            if ui
                                .selectable_value(
                                    &mut self.selected_timer,
                                    TimerOptions::TenSeconds,
                                    RichText::new("10 Seconds").size(30.0),
                                )
                                .clicked()
                            {
                                self.selected_timer_string = "10 Seconds".to_string();
                                self.selected_timer_numeric = 10;
                            };
                        });
                    if ui
                        .add_sized(
                            [50., 50.],
                            egui::Button::new(RichText::new("⚙ Settings").size(30.0)),
                        )
                        .clicked()
                    {
                        self.selected_window = 6;
                    }
                });
                ui.add_space(150.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
    
                    ui.label(
                        RichText::new(format!("{} to take a screenshot", self.shortcuts.get_hotkey_strings_formatted(1))).size(30.0).color(Color32::GRAY)
                        );
                });
            });
        } else if self.selected_window == 2 {
            self.hotkey_listener();
            frame.set_decorations(false);
            frame.set_window_size(self.screen_size.unwrap());
            frame.set_window_pos(self.frame_initial_pos.unwrap());

            self.multiplication_factor=frame.info().native_pixels_per_point;
                        
            match self.selected_mode {
                ModeOptions::Rectangle => {
                    egui::Area::new("my_area")
                        .fixed_pos(egui::pos2(0.0, 0.0))
                        .show(ctx, |ui| {
                            if self.current_os=="unix"{
                                ui.add_space(50.0);
                            }
                            
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.label(RichText::new(format!("{} to go back", self.shortcuts.get_hotkey_strings_formatted(0))).size(25.0).color(egui::Color32::WHITE));
                            });
                            ui.ctx()
                                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);

                            if ui.input(|i| {
                                i.pointer.any_down()
                                    && self.mouse_pos.unwrap()[0] == -1.0
                                    && self.mouse_pos.unwrap()[1] == -1.0
                            }) {
                                self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                                
                                }
                            if self.mouse_pos.unwrap()[0] != -1.0
                                && self.mouse_pos.unwrap()[1] != -1.0
                            {
                                self.mouse_pos_f = ui.input(|i| i.pointer.latest_pos());
                                self.define_rectangle();
                            }
                            if ui.input(|i| i.pointer.any_released()) {
                                frame.set_window_size(Vec2::new(0.0, 0.0));

                                self.selected_window = 3; 
                            }

                            ui.painter().add(Shape::Rect(RectShape::new(
                                Rect::from_min_max(self.rect_pos, self.rect_pos_f),
                                Rounding::default(),
                                Color32::TRANSPARENT,
                                Stroke::new(2.0, Color32::GRAY),
                            )));
                        });
                }
                ModeOptions::FullScreen => {
                    frame.set_window_size(Vec2::new(0.0, 0.0));

                    self.selected_window = 3;
                }
            }
        } else if self.selected_window == 3 {
            self.selected_window = 4;
        } else if self.selected_window == 4 {
            self.take_screenshot();
            self.selected_window = 5;
        } else if self.selected_window == 5 {
            
            self.hotkey_listener();
            frame.set_decorations(true);
            if self.first_time{
                frame.set_window_pos(Pos2{x: 0.0, y: 0.0});
                self.first_time=false;
            }
            
            
            if self.current_os=="windows"{
                frame.set_window_size(self.screen_size.unwrap()/(self.multiplication_factor.unwrap()));
            }else{
                frame.set_window_size(self.screen_size.unwrap());
            }
            
            //println!("w={:} , h={:}",self.width,self.height);
           
               /*  
               
            if self.width <= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(1100.0, 600.0)); //1400 750
                println!("1");
            } else if self.width <= 1000.0 && self.height >= 500.0 {
                frame.set_window_size(Vec2::new(1100.0, self.height+self.height*0.3));
                println!("2");
            } else if self.width >= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(self.screen_size.unwrap().x /self.multiplication_factor.unwrap(), 600.0));
                println!("3");
            } else if self.width >= 1200.0 && self.height >= 700.0 {
                println!("4");
                frame.set_window_size(Vec2::new(1300.0, 800.0));
            } else {
                println!("5");
                frame.set_window_size(Vec2::new(self.screen_size.unwrap().x /self.multiplication_factor.unwrap()- self.screen_size.unwrap().x /self.multiplication_factor.unwrap()*0.001, self.screen_size.unwrap().y /self.multiplication_factor.unwrap()- self.screen_size.unwrap().y /self.multiplication_factor.unwrap()*0.01));
            }

           */

            let mut paint_btn = None;
            let mut text_btn = None;
            let mut save_btn = None;
            let mut save_edit_btn = None;
            let mut copy_btn=None;
            let mut crop_btn=None;
            let mut finish_crop=None;
            let mut settings_btn=None;
            let mut exit_cut_btn=None;
            
            if self.show_toast{
                self.toasts.as_mut().unwrap().show(ctx);                    
            }
            
            egui::CentralPanel::default().show(ctx, |_ui| {
                
                
                egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
                    
                    ui.horizontal(|ui| {
                        if self.cut_clicked==false{

                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0); 
                                paint_btn = Some(ui.add(egui::Button::new(RichText::new("🖊 Paint").size(20.0))));
                            }
                        ) ;  

                        if paint_btn.unwrap().clicked() {
                            self.pp_option = Some(PpOptions::Painting);
                            self.selected_shape_string = "Select a shape!".to_string();
                            self.ready_to_cut=None;
                        }
                        egui::ComboBox::from_id_source("Select a shape!")
                            .selected_text(RichText::new(format!("{}", self.selected_shape_string)).size(20.0))
                            
                            .show_ui(ui, |ui| {
                               
                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Arrow,
                                        RichText::new("↘ Arrow").size(20.0),
                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Arrow;
                                    self.selected_shape_string = "↘ Arrow".to_string();
                                    self.pp_option = Some(PpOptions::Arrow);
                                    self.ready_to_cut=None;
                                }

                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Circle,
                                        RichText::new("⭕ Circle").size(20.0),
                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Circle;
                                    self.selected_shape_string = "⭕ Circle".to_string();
                                    self.pp_option = Some(PpOptions::Circle);
                                    self.ready_to_cut=None;

                                }

                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Square,
                                        RichText::new("⬜ Square").size(20.0),

                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Square;
                                    self.selected_shape_string = "⬜ Square".to_string();
                                    self.pp_option = Some(PpOptions::Square);
                                    self.ready_to_cut=None;
                                };
                            });

                            ui.vertical(
                                |ui| {
                                    ui.add_space(6.0); 
                                     text_btn = Some(ui.add(egui::Button::new(RichText::new("✒ Text").size(20.0))));
                       
                                }
                            ) ;  
                       if text_btn.unwrap().clicked() {
                            self.pp_option = Some(PpOptions::Text);
                            self.selected_shape_string = "Select a shape!".to_string();
                            self.ready_to_cut=None;
                        }
                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0);
                                save_btn = Some(ui.add(egui::Button::new(RichText::new("Save").size(20.0))).on_hover_text(self.shortcuts.get_hotkey_strings_formatted(2)));
                      
                            }
                        ) ;  
                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0); 
                                 save_edit_btn = Some(ui.add(egui::Button::new(RichText::new("Save with name").size(20.0))).on_hover_text(self.shortcuts.get_hotkey_strings_formatted(4)));
                        
                            }
                        ) ;  
                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0);
                                copy_btn = Some(ui.add(egui::Button::new(RichText::new("Copy").size(20.0))).on_hover_text(self.shortcuts.get_hotkey_strings_formatted(3)));
                    
                            }
                        ) ;  
                        }
                        
                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0);
                                crop_btn=Some(ui.add_enabled((!self.cut_clicked && self.dim_bool),egui::Button::new(RichText::new("Cut").size(20.0))).on_hover_text(self.shortcuts.get_hotkey_strings_formatted(5)));
                        
                            }
                        ) ;  
                       
                        if self.cut_clicked==false{
                            
                        ui.add_space(90.0*ui.style().spacing.item_spacing.x);

                        ui.vertical(
                            |ui| {
                                ui.add_space(6.0);
                                settings_btn=Some(ui.add(egui::Button::new(RichText::new("⚙ Settings").size(20.0))));

                            }
                        ) ;  
                    
                        }

                    });
                   
                    match self.loading_state {                        
                        LoadingState::Loaded => {
                            let dim: Vec2;
                            if self.width >= 1200.0 && self.height >= 700.0 {
                               //println!("caso 1");
                                if self.current_os=="windows"{
                                    self.shrink_fact=Some(0.6/self.multiplication_factor.unwrap());
                                }else{
                                    self.shrink_fact=Some(0.6);
                                }
                              
                                dim = Vec2::new(self.width*self.shrink_fact.unwrap(), self.height*self.shrink_fact.unwrap()); 
                            } else if self.width >= 1200.0 && self.height <= 700.0 {
                                //println!("caso 2");
                                if self.current_os=="windows"{
                                    self.shrink_fact=Some(0.65/self.multiplication_factor.unwrap());
                                }else{
                                    self.shrink_fact=Some(0.65);
                                }
                                
                                dim = Vec2::new(self.width*self.shrink_fact.unwrap(), self.height*self.shrink_fact.unwrap());
                            } else if self.width <= 1200.0 && self.height >= 700.0 {   
                               // println!("caso 3");    
                                if self.current_os=="windows"{
                                    self.shrink_fact=Some(0.6/self.multiplication_factor.unwrap());
                                }else{
                                    self.shrink_fact=Some(0.6);
                                }                                                 
                               
                                dim = Vec2::new(self.width*self.shrink_fact.unwrap() , self.height*self.shrink_fact.unwrap());
                            } else {
                               //println!("caso 4");
                                if self.current_os=="windows"{
                                    self.shrink_fact=Some(1.0/self.multiplication_factor.unwrap());
                                }else{
                                    self.shrink_fact=Some(1.0);
                                }
                            

                                dim = Vec2::new(self.width * self.shrink_fact.unwrap(), self.height*self.shrink_fact.unwrap());
                            }

                            if self.shrink_fact.is_some(){
                             if dim[0]>30.0*self.shrink_fact.unwrap() || dim[1]>30.0*self.shrink_fact.unwrap(){
                                 self.dim_bool=true;
                             }else{
                                 self.dim_bool=false;
                             } 
                            }

                            let mut pxs = None;
                            let mut arr=None;
                            let mut txt = None;
                            let mut sqrs = None;
                            let mut crcls=None;
                            let mut response=None;
                            
                            

                            (pxs, arr, txt, sqrs, crcls,response) = self
                                .painting
                                .ui(
                                    ui,
                                    egui::Image::new(self.image.as_ref().unwrap()).shrink_to_fit(),
                                    &mut self.mult_factor,
                                    dim,
                                    self.pp_option.clone().unwrap(),
                                    self.save,
                                    self.cut_clicked,
                                )
                                .clone();
                                
                                  
                                self.save=false;
                                self.cropped=false;
                                if pxs.is_none() == false {
                                                 self.line_pixels = pxs.clone().unwrap();
                                           }
                                           if arr.is_none() == false {
                                           
                                                    self.arrow_pixels=arr.clone().unwrap();
                                                        
                                                    }
                                            if crcls.is_none() == false {
                                                            self.circle_pixels = crcls.clone().unwrap();
                                                            
                                                        }
                                                if sqrs.is_none() == false {
                                                                self.square_pixels = sqrs.clone().unwrap();
                                                            }
                                                    if txt.is_none() == false {
                                                                    self.text_pixels=txt.clone().unwrap();
                                                                }
                                // match self.pp_option.clone().unwrap() {
                                //     PpOptions::Painting => {
                                //         if pxs.is_none() == false {
                                //             self.line_pixels = pxs.clone().unwrap();
                                //         }
                                //     }
                                //     PpOptions::Arrow => {
                                //         if arr.is_none() == false {
                                           
                                //         self.arrow_pixels=arr.clone().unwrap();
                                            
                                //         }
                                //     }
                                //     PpOptions::Circle => {
                                //         if crcls.is_none() == false {
                                //             self.circle_pixels = crcls.clone().unwrap();
                                //             println!("cerchi nel main {:?}", self.circle_pixels.len());
                                //         }
                                //     }
                                //     PpOptions::Square => {
                                //         if sqrs.is_none() == false {
                                //             self.square_pixels = sqrs.clone().unwrap();
                                //         }
                                //     }
                                //     PpOptions::Text => {
                                //         if txt.is_none() == false {
                                //             self.text_pixels=txt.clone().unwrap();
                                //         }
                                //     }
                                //     PpOptions::Cut => {
                                        
                                //     }
                                // }

                            if (save_btn.is_none()==false && save_btn.unwrap().clicked() )|| self.ready_to_save {

                                self.save_img(ui);
                                self.pp_option=Some(PpOptions::Painting);
                                self.ready_to_save = false;
                                self.ready_to_cut=None;


                                self.edit_image(ui);
                                self.circle_pixels= Vec::new();
                                self.square_pixels= Vec::new();
                                self.arrow_pixels= Vec::new();
                                self.text_pixels= Vec::new();
                                self.line_pixels=  Vec::new();
                                

                            }
                            if (save_edit_btn.is_none()==false && save_edit_btn.unwrap().clicked() )|| self. ready_to_save_with_name{
                                self.edit_image(ui);
                                let dialog = FileDialog::new().add_filter(".jpg", &["jpg"]).add_filter(".png", &["png"]).add_filter(".gif", &["gif"]).save_file();
                                
                                let mut stringpath: String;
                                self.save=true;
                                self.ready_to_cut=None;
                                
                                if dialog.is_some(){
                                stringpath =  dialog
                                .clone()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap()
                                .to_string();
                                 let slice = stringpath.get(..stringpath.len() -3).unwrap();
                                self.toasts.as_mut().unwrap().success(format!(
                                    "Image saved in {}{}",
                                    slice,
                                    self.image_format_string
                                   
                                )).set_duration(Some(Duration::from_secs(5)));
                                

                                self.show_toast=true;
                                
                        

 

  
                                let mod_img: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> = self.image_buffer.clone();
                                if mod_img.is_none() == false {

                                let slice = stringpath.get(..stringpath.len() -3).unwrap();

                                    let _ = mod_img.unwrap().save(format!(
                                        "{}{}",
                                        slice,
                                        self.image_format_string
                                       
                                    ));
                                }
                                self.save = true;
                                self.ready_to_save_with_name = false;
                                self.pp_option=Some(PpOptions::Painting);
                                
                                self.circle_pixels= Vec::new();
                                self.square_pixels= Vec::new();
                                self.arrow_pixels= Vec::new();
                                self.text_pixels= Vec::new();
                                self.line_pixels=  Vec::new();
  

                                
                             }
                            }
                            if (copy_btn.is_none()==false && copy_btn.unwrap().clicked()) || self.ready_to_copy{
                                self.edit_image(ui);
                                self.toasts.as_mut().unwrap().success("Image copied to clipboard" ).set_duration(Some(Duration::from_secs(5)));
                                self.ready_to_cut=None;
                                self.show_toast=true;
                                let mut clipboard = Clipboard::new().unwrap();
                                let w=self.image_buffer.clone().unwrap().width() as usize;
                                let h=self.image_buffer.clone().unwrap().height() as usize;
                                let _=clipboard.set_image(arboard::ImageData { width: w, height: h, bytes: self.image_buffer.clone().unwrap().as_bytes().into()});
                                self.ready_to_copy=false;
                            }

                            if !self.ready_to_cut.is_none() && self.ready_to_cut.unwrap()==false {
                                ui.ctx()
                                .output_mut(|i| i.cursor_icon = CursorIcon::Default);
                           
                                egui::Window::new("Saving")
            //.constraint_to(response.clone().unwrap().rect)
            .default_width(300.0)//da modificare
            .default_height(50.0)//da modificare
            .title_bar(false)
            .default_pos(Pos2::new((frame.info().window_info.size.x/2.0)/self.multiplication_factor.unwrap(), (frame.info().window_info.size.y/2.0)/self.multiplication_factor.unwrap()))
            .vscroll(false)
            .interactable(true)
            .resizable(true)
            // .frame(egui::Frame::none()
            //      .fill(egui::Color32::from_rgba_unmultiplied(70, 70, 70, 70))
            //      .stroke(Stroke::new(1.0, egui::Color32::WHITE))
            //      )
            .show(ctx, |ui| {
                ui.label(RichText::new("Your changes will be saved, do you want to proceed?").color(Color32::WHITE).size(15.0));
                let mut yes_btn=Some(ui.add(egui::Button::new("Yes")));
                let mut no_btn=Some(ui.add(egui::Button::new("No")));

                if yes_btn.unwrap().clicked(){
                    
                    self.ready_to_cut=Some(true);
                    self.ready_to_crop=true;
                   //self.selected_window=5;

                }

                if no_btn.unwrap().clicked(){
                    self.ready_to_cut=None;
                    //self.selected_window=5;
                }
            });
                              }
                           
                            if (crop_btn.unwrap().clicked() || self.cut_clicked==true)||self.ready_to_crop{   

                                ui.horizontal(|ui|{
                                    finish_crop=Some(ui.add_enabled(self.cut_clicked, egui::Button::new(RichText::new("Finish Your Cut").size(20.0))));
                                exit_cut_btn=Some(ui.add_enabled(self.cut_clicked, egui::Button::new(RichText::new("Exit").size(20.0))));
                                  
                                });
                                                       
                                if self.ready_to_cut.is_none() && (self.square_pixels.len()>0 || self.text_pixels.len()>0 || self.circle_pixels.len()>0 || self.line_pixels.len()>0 || self.arrow_pixels.len()>0){
                                    self.ready_to_cut=Some(false);
                                }

                                     println!("{:?}", self.ready_to_cut);           
                                
                                if (self.ready_to_cut.is_none()==false && self.ready_to_cut.unwrap()==true) || (self.square_pixels.len()==0 && self.text_pixels.len()==0 && self.circle_pixels.len()==0 && self.line_pixels.len()==0 && self.arrow_pixels.len()==0) {
                                     
                                
                                self.pp_option = Some(PpOptions::Cut);
                                self.cut_clicked=true;
                                
                                if self.arrow_pixels.len()>0
                                    || self.circle_pixels.len()>0
                                    || self.square_pixels.len()>0
                                    || self.text_pixels.len()>0
                                    || self.line_pixels.len()>0{
                                        self.edit_image(ui);
                                        self.circle_pixels= Vec::new();
                                        self.square_pixels= Vec::new();
                                        self.arrow_pixels= Vec::new();
                                        self.text_pixels= Vec::new();
                                        self.line_pixels=  Vec::new();
     
                                    }
                                
                                let mut pos_bug_fixed=Pos2::new(0.0,0.0);

                                if ui.input(|i| i.pointer.hover_pos().is_none()==false){
                                    
                                    ui.input(|i| 
                                        pos_bug_fixed=i.pointer.hover_pos().unwrap()
                                        );
                                }

                                
                                if   pos_bug_fixed.x<=response.clone().unwrap().rect.right_top().x &&
                                     pos_bug_fixed.x>=response.clone().unwrap().rect.left_top().x &&
                                     pos_bug_fixed.y>=response.clone().unwrap().rect.left_top().y &&
                                     pos_bug_fixed.y<=response.clone().unwrap().rect.right_bottom().y {
                                        self.is_pointer_on_cut_window = true;
                                     }
                                     else {
                                        self.is_pointer_on_cut_window = false;
                                     }
                                
                            
                               
                                
                               let d= egui::Window::new("cut")
                               
                                .constraint_to(response.clone().unwrap().rect)
                                .min_width(30.0*self.shrink_fact.unwrap())
                                .min_height(30.0*self.shrink_fact.unwrap())
                                .default_width(dim[0]-1.0)//da modificare
                                .default_height(dim[1]-1.0)//da modificare
                                .title_bar(false)
                                .default_pos(Pos2::new(response.clone().unwrap().rect.left_top().x+1.0, response.clone().unwrap().rect.left_top().y+1.0))
                                .vscroll(false)
                                .interactable(   self.is_pointer_on_cut_window )
                                .resizable(   self.is_pointer_on_cut_window )
                                .frame(egui::Frame::none()
                                     .fill(egui::Color32::from_rgba_unmultiplied(70, 70, 70, 70))
                                     .stroke(Stroke::new(1.0, egui::Color32::WHITE))
                                     )
                                .show(ctx, |ui| {
                                     //2 linee verticali
                                     if self.set_Wh_window{
                                        println!("cambio w e h");
                                        ui.set_height(dim[1]-1.0);
                                        ui.set_width(dim[0]-1.0);
                                        
                                        self.set_Wh_window=false;
                                     }
                                     //println!("{:?}",  ui.available_size());
                                   
                                     ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    //2 linee orizzontali
                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));
                                    self.to_cut_rect= Some((ui.available_rect_before_wrap().left_top(), ui.available_rect_before_wrap().right_bottom()));
                                    
                                    ui.allocate_space(ui.available_size());
                                    
                                });
                                println!("response {:?}", response.clone().unwrap().rect.left_top());
                                println!("finestra {:?}", d.as_ref().unwrap().response.rect.left_top());
                                println!("response dim {:?}", response.clone().unwrap().rect.size());
                                println!("finestra dim {:?}", d.as_ref().unwrap().response.rect.size());

                               
                            
                                if exit_cut_btn.unwrap().clicked(){
                                    self.cut_clicked=false;
                                    self.pp_option=Some(PpOptions::Painting);
                                    self.ready_to_crop= false;
                                    self.ready_to_cut=None;
                                    self.selected_shape_string = "Select a shape!".to_string();
                                    self.cropped=false;
                                }

                                if finish_crop.unwrap().clicked(){
                                   
                                    self.set_Wh_window=true;
                                    self.cut_clicked=false;
                                    self.load_cutted_img(ui, response);
                                    self.cropped=true;
                                    self.pp_option=Some(PpOptions::Painting);
                                    self.ready_to_crop= false;
                                    self.ready_to_cut=None;
                                    self.selected_shape_string = "Select a shape!".to_string();

                                }
                               

                            }
                            }else if dim[0]==30.0 && dim[1]==30.0{
                                self.cut_clicked=false;
                            }
                            if (settings_btn.is_none()==false && settings_btn.unwrap().clicked()){
                                self.selected_window=6;
                            }
                        }
                        LoadingState::NotLoaded => {
                            
                                if self.image_texture.is_none()==false && (self.width>50.0 && self.height>50.0){
                                    self.load_image(ui);
                                    self.pp_option = Some(PpOptions::Painting);
                                    self.loading_state = LoadingState::Loaded;

                                }else{
                                    self.selected_window=1;
                                }
                                
                                ()
                            
                        }
                    }
                });
               
            });
        } else if self.selected_window == 6 {
           
            if self.show_toast{                
                self.toasts.as_mut().unwrap().show(ctx);    
            }
           
            let screens=Screen::all().unwrap();
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("Choose Path").clicked() {
                    self.filepath = FileDialog::new()
                        .set_directory("./screenshot")
                        .pick_folder();
                }
                ui.add_space(10.0);
                ui.heading(RichText::new("Select a format").color(Color32::WHITE));
                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Jpg),
                        "jpg",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Jpg);
                    self.image_format_string = "jpg".to_string();
                }
                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Png),
                        "png",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Png);
                    self.image_format_string = "png".to_string();
                }
                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Gif),
                        "gif",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Gif);
                    self.image_format_string = "gif".to_string();
                }
                ui.add_space(10.0);
                ui.heading(RichText::new("Select a monitor").color(Color32::WHITE));
                if ui
                    .add(egui::RadioButton::new(
                        self.screen_to_show==Some(screens[0].display_info.id),
                        "Primary",
                    ))
                    .clicked()
                {
                    self.screen_to_show=Some(screens[0].display_info.id);
                    self.screen_size=Some(Vec2::new(screens[0].display_info.width as f32, screens[0].display_info.height as f32));
                    self.frame_initial_pos=Some(Pos2::new(screens[0].display_info.x as f32, screens[0].display_info.y as f32));
                    
                }
                if screens.len()==2{
                    if ui
                    .add(egui::RadioButton::new(
                        self.screen_to_show==Some(screens[1].display_info.id),
                        "Secondary",
                    ))
                    .clicked()
                {
                    self.screen_to_show=Some(screens[1].display_info.id);
                    self.screen_size=Some(Vec2::new(screens[1].display_info.width as f32, screens[1].display_info.height as f32));
                    self.frame_initial_pos=Some(Pos2::new(screens[1].display_info.x as f32, screens[1].display_info.y as f32));
                   
                }
                }
               
                ui.add_space(10.0);
                egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.heading(RichText::new("Customizable Shortcuts").color(Color32::WHITE));
                    


                    ui.end_row();
                             if ui.button("Exit button").clicked(){
                                self.customizing_hotkey=0;      
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(0)).color(Color32::GRAY));
                            ui.end_row();  

                            if ui.button("Screenshot button").clicked(){
                                self.customizing_hotkey=1;
                                
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(1)).color(Color32::GRAY));
                            ui.end_row();
                            if ui.button("Save button").clicked(){
                                self.customizing_hotkey=2;                                
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(2)).color(Color32::GRAY));
                            ui.end_row();
                            if ui.button("Save with Name button").clicked(){
                                self.customizing_hotkey=3;
                                
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(3)).color(Color32::GRAY));
                            ui.end_row();
                            if ui.button("Copy button").clicked(){
                                self.customizing_hotkey=4;
                                
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(4)).color(Color32::GRAY));
                            ui.end_row();
                            if ui.button("Crop button").clicked(){
                                self.customizing_hotkey=5;
                                
                            }
                            ui.label(RichText::new(self.shortcuts.get_hotkey_strings_formatted(5)).color(Color32::GRAY));
                            ui.end_row();
                            
                            if self.customizing_hotkey != usize::MAX{
                                ui.ctx().output_mut(|i| i.cursor_icon = CursorIcon::Wait);
                                let mut ret=self.customize_shortcut(ui);
                                
                                if ret==1{
                                    self.toasts.as_mut().unwrap().success("Shortcut changed!" ).set_duration(Some(Duration::from_secs(5))); 
                                     self.show_toast=true;            
                                }else if ret==2{                                    
                                    self.toasts.as_mut().unwrap().error("Shortcut already used or invalid!" ).set_duration(Some(Duration::from_secs(5)));
                                    self.show_toast=true; 
                                }else if ret==3{
                                    
                                    self.toasts.as_mut().unwrap().error("Too many digits for the shortcut").set_duration(Some(Duration::from_secs(5)));
                                    self.show_toast=true;
                                    self.customizing_hotkey=usize::MAX;
                                }
                                
                            }
                           

                });
                
                ui.label(RichText::new("First click on the shortcut to modify and then press a combination of CTRL, ALT or SHIFT and a letter").size(15.0));
              
                ui.add_space(20.0);
                if ui.button("Back").clicked() {
                    if self.image.is_none(){
                        self.selected_window = 1;
                    }else if !self.image.is_none(){
                        self.selected_window=5;
                    }
                    
                }
            });
        }

    }
}

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const WINSIZEX: usize = 320;
const WINSIZEY: usize = 180;
const PIXELUPSCALE: usize = 5;

thread_local!(
    static GLOBAL: RefCell<Option<Window>> = RefCell::new(None);
);
fn check_update_display(){
    GLOBAL.with(|global|{
        if let Some(win) = &*global.borrow() {
            win.queue_draw();
        }
    })
}

fn x_y_in_vram(x: usize, y: usize) -> usize {
    return y * WINSIZEX + x
}


#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r: r, g: g, b: b }
    }
}

pub struct WindowClass {
    pub sizeY: usize,
    pub sizeX: usize,
    pub title: String,
    pub VRAM: [Color; WINSIZEY*WINSIZEX],
}
impl WindowClass {
    pub fn new(sizex: usize, sizey: usize, title: &str) -> Self {
        WindowClass {
            sizeX: sizex,
            sizeY: sizey,
            title: title.to_string(),
            VRAM: [Color::new(0.0, 0.0, 0.0); WINSIZEY*WINSIZEX],
        }
    }
    pub fn info(&self) {
        println!("`{}` - {}:{}", self.title, self.sizeX, self.sizeY);
    }

    pub fn init_drawing(&self) {
        gtk::init().expect("Failed to initialize GTK");
        let window: Window = Window::new(WindowType::Toplevel);
        let drawing_area = DrawingArea::new();
        window.set_title(self.title.as_str());
        window.set_default_size(WINSIZEX as i32 * PIXELUPSCALE as i32, WINSIZEY as i32 * PIXELUPSCALE as i32);
        window.set_position(gtk::WindowPosition::Center);
        window.connect_delete_event(|_, _| {gtk::main_quit(); Inhibit(false)});
        window.set_resizable(false);
        window.show();
        window.add(&drawing_area);
        window.show_all();
        let mut vram = self.VRAM;

        // Imitation of the ray-tracing fn
        //let mut color;
        for i in 0..WINSIZEY {
            for j in 0..WINSIZEX {
                //color = Color::new(0.0, ((100.0 * j as f64) / WINSIZEX as f64) / 100.0, 0.0);
                //color = Color::new(0.0, 0.5, 0.0);
                //println!("{}:{} - {} - {}", j, i, x_y_in_vram(j as usize, i as usize), color.g);
                vram[x_y_in_vram(j as usize, i as usize)] = Color::new(((100.0 * i as f64) / WINSIZEY as f64) / 100.0, ((100.0 * j as f64) / WINSIZEX as f64) / 100.0, 0.0);
            }
        }

        drawing_area.connect_draw(move |_, cr| {
            let mut err;
            let mut color: Color;
            for i in 0..WINSIZEY {
                for j in 0..WINSIZEX {
                    color = vram[x_y_in_vram(j, i)];
                    cr.set_source_rgb(color.r, color.g, color.b);
                    cr.rectangle(j as f64 * PIXELUPSCALE as f64, i as f64 * PIXELUPSCALE as f64, PIXELUPSCALE as f64, PIXELUPSCALE as f64);
                    err = cr.fill();
                    if err != Ok(()) {panic!("Error with drawing VRAM");}
                }
            }
            Inhibit(false)
        });

        /*drawing_area.connect_draw(move |_, cr| {
            let mut vram = vram_mut.lock().unwrap();
            let mut err;

            for i in 0..WINSIZEY*WINSIZEX {
                let x = i % WINSIZEX;
                let y = i / WINSIZEY;
                let color = vram[];
                cr.set_source_rgb(color.r, color.g, color.b);
                cr.rectangle(
                    (x * pixel_size) as f64,
                    (y * pixel_size) as f64,
                    pixel_size as f64,
                    pixel_size as f64,
                );
                err = cr.fill();
                if err != Ok(()) {
                    ERR::err_catch(ERR::Err::new(
                        "Failed to draw pixel, GTK error; {}",
                        ERR::LEVEL_ERR_LOG,
                    ));
                }
            }

            drop(vram);

            Inhibit(false)
        });
        */

        window.connect_event(|w, _| {
            w.queue_draw();
            Inhibit(false)
        });
        window.present();
        window.set_keep_above(true);

        GLOBAL.with(|global|{
            *global.borrow_mut() = Some(window);
        });

        glib::timeout_add(Duration::from_millis(100), move || {
            check_update_display();
            glib::Continue(true)
        });

        gtk::main();
    }
}

/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![feature(fn_traits, unboxed_closures)]

extern crate chrono;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate tokio_core;

use std::time::Duration;

use chrono::Local;
use gtk::{ContainerExt, Label, WidgetExt, Window, WindowType};
use relm::{Relm, RemoteRelm, Widget};
use tokio_core::reactor::Interval;

use self::Msg::*;

#[derive(SimpleMsg)]
enum Msg {
    Quit,
    Tick,
}

struct Widgets {
    label: Label,
    window: Window,
}

struct Win {
    widgets: Widgets,
}

impl Win {
    fn view(relm: &RemoteRelm<Msg>) -> Widgets {
        let label = Label::new(None);

        let window = Window::new(WindowType::Toplevel);

        window.add(&label);

        window.show_all();

        connect_no_inhibit!(relm, window, connect_delete_event(_, _), Quit);

        Widgets {
            label: label,
            window: window,
        }
    }
}

impl Widget<Msg> for Win {
    type Container = Window;
    type Model = ();

    fn container(&self) -> &Self::Container {
        &self.widgets.window
    }

    fn new(relm: RemoteRelm<Msg>) -> (Self, ()) {
        let widgets = Self::view(&relm);
        let mut win = Win {
            widgets: widgets,
        };
        win.update(Tick, &mut ());
        (win, ())
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let stream = Interval::new(Duration::from_secs(1), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(stream, Tick);
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Tick => {
                let time = Local::now();
                self.widgets.label.set_text(&format!("{}", time.format("%H:%M:%S")));
            },
            Quit => gtk::main_quit(),
        }
    }
}

fn main() {
    Relm::run::<Win>().unwrap();
}
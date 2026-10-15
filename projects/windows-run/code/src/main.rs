use gtk4 as gtk;
use gtk::{gdk, gio, glib, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::{
    cell::Cell,
    fs,
    io::Write,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Command, Stdio},
    rc::Rc,
};

const LEFT_MARGIN: i32 = 48;
const BOTTOM_MARGIN: i32 = 48;

const CSS: &str = r#"
window {
    background: #f0f0f0;
    border: 1px solid #0078d7;
    color: #000;
    font-family: "Segoe UI", "Selawik", "Noto Sans", "DejaVu Sans", sans-serif;
    font-size: 9pt;
}
.titlebar { background: #fff; min-height: 30px; padding-left: 8px; }
.titlebar label { padding-left: 8px; }
button.close {
    background: transparent; background-image: none; border: none;
    border-radius: 0; box-shadow: none; padding: 0;
    min-width: 46px; min-height: 30px;
}
button.close:hover, button.close:hover * { background: #e81123; color: #fff; }
.body { padding: 16px 12px 12px 12px; }
entry {
    background: #fff; border: 1px solid #7a7a7a; border-radius: 0;
    box-shadow: none; outline: none; min-height: 21px; padding: 0 3px;
}
entry:focus-within { border-color: #0078d7; }
button {
    background: #e1e1e1; background-image: none; border: 1px solid #adadad;
    border-radius: 0; box-shadow: none; padding: 0;
    min-width: 75px; min-height: 23px;
}
button:hover  { background: #e5f1fb; border-color: #0078d7; }
button:active { background: #cce4f7; border-color: #005499; }
button.default { border: 2px solid #0078d7; }
"#;

fn hist_path() -> PathBuf {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        });
    base.join("winrun.hist")
}

fn load_history() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(s) = fs::read_to_string(hist_path()) {
        for l in s.lines().rev() {
            if !l.is_empty() && !out.iter().any(|x| x == l) {
                out.push(l.to_string());
            }
            if out.len() >= 50 {
                break;
            }
        }
    }
    out
}

fn save_history(cmd: &str) {
    let p = hist_path();
    if let Some(d) = p.parent() {
        let _ = fs::create_dir_all(d);
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{cmd}");
    }
}

fn run(cmd: &str) {
    let cmd = cmd.trim();
    if cmd.is_empty() {
        return;
    }
    save_history(cmd);
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn();
}

#[allow(deprecated)]
fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("no display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &gtk::Application) {
    load_css();

    let win = gtk::ApplicationWindow::new(app);
    win.set_decorated(false);
    win.set_resizable(false);

    win.init_layer_shell();
    win.set_namespace("winrun");
    win.set_layer(Layer::Top);
    win.set_anchor(Edge::Left, true);
    win.set_anchor(Edge::Bottom, true);
    win.set_margin(Edge::Left, LEFT_MARGIN);
    win.set_margin(Edge::Bottom, BOTTOM_MARGIN);
    win.set_keyboard_mode(KeyboardMode::Exclusive);

    let title = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    title.add_css_class("titlebar");
    let ticon = gtk::Image::from_icon_name("system-run");
    ticon.set_pixel_size(16);
    let tlabel = gtk::Label::new(Some("Run"));
    tlabel.set_hexpand(true);
    tlabel.set_xalign(0.0);
    let close = gtk::Button::with_label("✕");
    close.add_css_class("close");
    title.append(&ticon);
    title.append(&tlabel);
    title.append(&close);

    let top = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let big = gtk::Image::from_icon_name("system-run");
    big.set_pixel_size(32);
    big.set_valign(gtk::Align::Start);
    let desc = gtk::Label::new(Some(
        "Type the name of a program, folder, document, or Internet resource, and Windows will open it for you.",
    ));
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_width_chars(42);
    desc.set_max_width_chars(42);
    top.append(&big);
    top.append(&desc);

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_margin_top(14);
    let open = gtk::Label::new(None);
    open.set_markup("<u>O</u>pen:");
    let entry = gtk::Entry::new();
    entry.set_hexpand(true);
    row.append(&open);
    row.append(&entry);

    let btns = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    btns.set_halign(gtk::Align::End);
    btns.set_margin_top(18);
    let ok = gtk::Button::with_label("OK");
    ok.add_css_class("default");
    let cancel = gtk::Button::with_label("Cancel");
    let browse = gtk::Button::with_label("Browse...");
    btns.append(&ok);
    btns.append(&cancel);
    btns.append(&browse);

    let body = gtk::Box::new(gtk::Orientation::Vertical, 0);
    body.add_css_class("body");
    body.append(&top);
    body.append(&row);
    body.append(&btns);

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.set_size_request(412, -1);
    root.append(&title);
    root.append(&body);
    win.set_child(Some(&root));

    let go: Rc<dyn Fn()> = {
        let e = entry.clone();
        let app = app.clone();
        Rc::new(move || {
            run(e.text().as_str());
            app.quit();
        })
    };
    {
        let go = go.clone();
        ok.connect_clicked(move |_| go());
    }
    {
        let go = go.clone();
        entry.connect_activate(move |_| go());
    }
    {
        let app = app.clone();
        cancel.connect_clicked(move |_| app.quit());
    }
    {
        let app = app.clone();
        close.connect_clicked(move |_| app.quit());
    }
    {
        let e = entry.clone();
        let w = win.clone();
        browse.connect_clicked(move |_| {
            w.set_keyboard_mode(KeyboardMode::OnDemand);
            let dlg = gtk::FileDialog::new();
            let e = e.clone();
            dlg.open(None::<&gtk::Window>, gio::Cancellable::NONE, move |res| {
                if let Ok(f) = res {
                    if let Some(p) = f.path() {
                        e.set_text(&p.to_string_lossy());
                        e.set_position(-1);
                    }
                }
            });
        });
    }

    {
        let app = app.clone();
        let kc = gtk::EventControllerKey::new();
        kc.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                app.quit();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        win.add_controller(kc);
    }

    {
        let hist = Rc::new(load_history());
        let idx = Rc::new(Cell::new(-1i32));
        let e = entry.clone();
        let kc = gtk::EventControllerKey::new();
        kc.connect_key_pressed(move |_, key, _, _| {
            let n = hist.len() as i32;
            if n == 0 {
                return glib::Propagation::Proceed;
            }
            if key == gdk::Key::Up {
                let i = (idx.get() + 1).min(n - 1);
                idx.set(i);
                e.set_text(&hist[i as usize]);
                e.set_position(-1);
                glib::Propagation::Stop
            } else if key == gdk::Key::Down {
                let i = (idx.get() - 1).max(-1);
                idx.set(i);
                e.set_text(if i < 0 { "" } else { &hist[i as usize] });
                e.set_position(-1);
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        entry.add_controller(kc);
    }

    win.present();
    entry.grab_focus();
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("local.winrun")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

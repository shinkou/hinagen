extern crate gio;
extern crate glib;
extern crate gtk;
extern crate hinafont;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

fn main()
{
	let application = gtk::Application::new
	(
		Some("com.github.gtk-rs.examples.basic")
		, Default::default()
	).expect("Failed to initialize GTK application");

	application.connect_activate
	(|app|{
		let window = gtk::ApplicationWindow::new(app);
		window.set_title("Hinagen");
		window.set_default_size(350, 0);

		let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);

		let entry1 = gtk::Entry::new();
		let button1 = gtk::Button::new_with_label("Generate");

		hbox.pack_start(&entry1, true, true, 0);
		hbox.pack_end(&button1, false, false, 0);

		window.add(&hbox);
		window.show_all();

		let conv_text = |e: gtk::Entry|
		{
			match e.to_owned().get_text()
			{
				Some(gs) => println!("{}", hinafont::conv(gs.as_ref()))
				, None => ()
			}
			e.set_text("");
			e.grab_focus_without_selecting();
		};

		entry1.connect_activate
		(
			clone!(@weak entry1 => move |_| conv_text(entry1))
		);

		button1.connect_clicked
		(
			clone!(@weak entry1 => move |_| conv_text(entry1))
		);
	});

	application.run(&[]);
}

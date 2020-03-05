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
		window.set_default_size(256, 0);

		let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
		let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);

		let entry1 = gtk::Entry::new();
		let button1 = gtk::Button::new_with_label("Generate");
		let button2 = gtk::Button::new_with_label("Reset");

		let tview1 = gtk::TextView::new();
		tview1.set_editable(false);
		tview1.set_monospace(true);
		let swindow1 = gtk::ScrolledWindow::new
		(
			None::<&gtk::Adjustment>
			, None::<&gtk::Adjustment>
		);
		swindow1.set_policy
		(
			gtk::PolicyType::Automatic
			, gtk::PolicyType::Automatic
		);
		swindow1.set_size_request(256, 128);
		swindow1.add(&tview1);

		hbox.pack_start(&entry1, true, true, 0);
		hbox.pack_end(&button2, false, false, 0);
		hbox.pack_end(&button1, false, false, 0);

		vbox.pack_start(&hbox, false, false, 0);
		vbox.pack_end(&swindow1, true, true, 0);

		window.add(&vbox);
		window.show_all();

		let conv_text = |e: gtk::Entry, tv: gtk::TextView|
		{
			match e.get_text()
			{
				Some(gs) => match tv.get_buffer()
				{
					Some(buf) => buf.insert
					(
						&mut buf.get_end_iter()
						, &hinafont::conv(gs.as_ref())
					)
					, None => ()
				}
				, None => ()
			}
			e.set_text("");
			e.grab_focus_without_selecting();
		};

		let reset_bufs = |e: gtk::Entry, tv: gtk::TextView|
		{
			e.set_text("");
			match tv.get_buffer()
			{
				Some(buf) => buf.set_text("")
				, None => ()
			}
			e.grab_focus_without_selecting();
		};

		entry1.connect_activate
		(
			clone!
			(
				@weak entry1, @weak tview1
					=> move |_| conv_text(entry1, tview1)
			)
		);

		button1.connect_clicked
		(
			clone!
			(
				@weak entry1, @weak tview1
					=> move |_| conv_text(entry1, tview1)
			)
		);

		button2.connect_clicked
		(
			clone!
			(
				@weak entry1, @weak tview1
					=> move |_| reset_bufs(entry1, tview1)
			)
		);
	});

	application.run(&[]);
}

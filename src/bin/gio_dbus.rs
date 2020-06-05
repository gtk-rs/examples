extern crate futures;

extern crate glib;
use glib::ToVariant;

extern crate gio;
use gio::ApplicationExt;

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();

    let app = gio::Application::new(Some("org.gtk-rs.Demo"), gio::ApplicationFlags::default());
    app.register::<gio::Cancellable>(None).unwrap();

    // Get a D-Bus connection from a gio::Application
    let bus = app.get_dbus_connection().unwrap();

    // Make a call using an async future
    let result = c.block_on(bus.call_future(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
        "GetCapabilities",
        None,
        None,
        gio::DBusCallFlags::NONE,
        69,
    ));
    println!("Capabilities of the notification server: {:?}", result);

    // Load interface information for server-side operation
    let intf_info = gio::DBusNodeInfo::new_for_xml(r#"
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN" "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node>
	<interface name="org.gtk_rs.Demo">
	  <property name="One" type="u" access="read" />
	  <property name="Two" type="u" access="write" />
		<method name="Hello">
			<arg name="name" type="s" direction="in"/>
		</method>
	</interface>
</node>
"#).unwrap().lookup_interface("org.gtk_rs.Demo").unwrap();

		// Register object for the above interface (using closures to serve requests)
    bus.register_object(
        "/org/gtk_rs/Demo",
        &intf_info,
        |_conn, uniq, path, intf, meth, args, invo| {
            println!(
                "Server method call: {} {} {} {}: {:?}",
                uniq, path, intf, meth, args
            );
            // Have to return a value using this call:
            invo.return_value(None);
        },
        |_conn, uniq, path, intf, prop| {
            println!("Server property read: {} {} {} {}", uniq, path, intf, prop);
            // Return the value of the right property
            1337_i32.to_variant()
        },
        |_conn, uniq, path, intf, prop, val| {
            println!(
                "Server property set: {} {} {} {}: {:?}",
                uniq, path, intf, prop, val
            );
            // Return whether the write succeeded
            false
        },
    )
    .unwrap();

    println!("org.gtk_rs.Demo server running. Try interacting using e.g. D-Feet!");
    l.run();

    c.pop_thread_default();
}

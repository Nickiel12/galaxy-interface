use zbus::{SignalContext, ObjectServer, ConnectionBuilder, dbus_interface, fdo, Result};
use std::process::Command;

use event_listener::Event;
use async_std;

struct Greeter {
    name: String,
    done: Event,
}

#[dbus_interface(name = "org.galaxymenu.MyGreeter")]
impl Greeter {

    async fn next_desktop(&self) {
        Command::new("awesome-client")
        .arg("require(\"awful\").tag.viewnext()")
        .output()
        .expect("Failed to execute command");
    }

    async fn prev_desktop(&self) {
        Command::new("awesome-client")
        .arg("require(\"awful\").tag.viewprev()")
        .output()
        .expect("Failed to execute command");
    }

    async fn set_desktop(&self, desktop_num: u8) {
        Command::new("awesome-client")
        .arg(format!("local awful = require(\"awful\") \n local screen = awful.screen.focused()\n local tag = screen.tags[{}] \n if tag then \n tag:view_only()\n end", desktop_num))
        .output()
        .expect("Failed");
    }

    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    // Rude!
    async fn go_away(&self) {
        self.done.notify(1);
    }

    /// A "GreeterName" property.
    #[dbus_interface(property)]
    async fn greeter_name(&self) -> &str {
        &self.name
    }

    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.
    #[dbus_interface(property)]
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    /// A signal; the implementation is provided by the macro.
    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

// Although we use `async-std` here, you can use any async runtime of choice.



#[async_std::main]
async fn main() -> Result<()> {
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };
    let done_listener = greeter.done.listen();
    let _ = ConnectionBuilder::session()?
        .name("org.galaxymenu.MyGreeter")?
        .serve_at("/org/galaxymenu/MyGreeter", greeter)?
        .build()
        .await?;

    done_listener.wait();

    Ok(())
}

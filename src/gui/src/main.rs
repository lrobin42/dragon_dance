/* reference url: https://hackmd.io/@Hamze/Sys9nvF6Jl */

// Use `use` to bring items into the current scope.
// `eframe::egui` means we are accessing the `egui` module *within* the `eframe` crate.
// The `::` is a path separator, like `/` in file paths, but for code modules.
use eframe::egui;

// `fn main()` defines the main function, the entry point of every Rust executable.
// `-> Result<(), eframe::Error>` specifies the return type.
// `Result` is a standard Rust enum used for error handling. It can be either:
//  - `Ok(value)`: The operation succeeded, containing the value (here `()`, the empty tuple or "unit type", signifying no specific value).
//  - `Err(error_value)`: The operation failed, containing an error value (here, an `eframe::Error`).
// This means `main` can signal if it failed to start the eframe application.
fn main() -> Result<(), eframe::Error> {
    // `let options = ...;` declares a variable named `options`.
    // `eframe::NativeOptions::default()` calls an "associated function" (like a static method)
    // named `default` on the `NativeOptions` struct within the `eframe` crate.
    // The `Default` trait provides this standard way to get default values.
    let options = eframe::NativeOptions::default();

    // Call the `run_native` function from the `eframe` crate.
    eframe::run_native(
        "egui Demo", // Window title (a string literal)
        options,     // The options struct we just created
        // This part is a bit advanced, involving closures and trait objects:
        // `Box::new(|_cc| ...)`: Creates a closure (an anonymous function).
        //   `|_cc|` : Defines the closure's input argument (`_cc` for creation context, underscore means we don't use it).
        //   `Box::new(MyApp::default())`: Inside the closure, create a default `MyApp` instance.
        //   `Box::new(...)`: Allocates the `MyApp` instance on the heap and returns a "boxed" pointer.
        // Why `Box`? `run_native` needs to work with *any* type that implements `eframe::App`.
        // `Box<dyn eframe::App>` (which is what this effectively creates) allows this flexibility
        // by using dynamic dispatch via a "trait object". Don't worry too much about this for now,
        // just know it's the standard way to pass your app logic to eframe.
        Box::new(|_cc| Box::new(MyApp::default())),
    ) // The `?` operator could be added here (`run_native(...)?)` to automatically propagate errors if `run_native` returned an `Err`.
}

// `#[derive(Default)]` is an "attribute" that asks the compiler to automatically
// generate a default implementation for this struct. For `MyApp`, this means
// creating an instance where `label` is an empty `String` and `value` is `0.0`.
// We will modify this struct and its Default implementation later.
#[derive(Default)]
// `struct MyApp { ... }` defines a custom data structure named `MyApp`.
// It groups related data fields together. This holds our application's state.
struct MyApp {
    // `label: String,` defines a field named `label` of type `String` (a growable text string).
    label: String,
    // `value: f32,` defines a field named `value` of type `f32` (a 32-bit floating-point number).
    value: f32,
    // We'll add more fields later!
}

// `impl eframe::App for MyApp { ... }` starts an implementation block.
// It says: "We are implementing the `eframe::App` trait *for* our `MyApp` struct."
// A `trait` defines a set of methods that a type must provide (like an interface).
// `eframe::App` requires structs used with `run_native` to have methods like `update`.
impl eframe::App for MyApp {
    // `fn update(...) { ... }` defines the required `update` method for the `eframe::App` trait.
    // `&mut self`: Takes a mutable reference to the instance of `MyApp` this method is called on.
    //   `&`: Indicates a reference (borrowing) - we don't take ownership.
    //   `mut`: Indicates the reference is mutable - we are allowed to *change* the `MyApp` instance's fields.
    //   `self`: Refers to the specific `MyApp` instance being updated.
    // `ctx: &egui::Context`: Takes an immutable reference (`&`) to the `egui::Context`. We only need to read from it.
    // `_frame: &mut eframe::Frame`: A mutable reference to frame info (underscore `_` means we don't use this variable).
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // `egui::CentralPanel::default()` creates a default central panel configuration.
        // `.show(ctx, |ui| { ... })` calls the `show` method on the panel.
        //   `ctx`: Passes the egui context.
        //   `|ui| { ... }`: This is a closure! It's an anonymous function passed *to* `show`.
        //     `|ui|`: Defines the input argument for the closure, named `ui`. `egui` provides this `Ui` object.
        //     `{ ... }`: The body of the closure, containing the code that defines the UI using the `ui` object.
        egui::CentralPanel::default().show(ctx, |ui| {
            // `ui` is of type `&mut egui::Ui`. It's a mutable reference, so methods called on `ui` can change its internal state (e.g., layout position).
            // `ui.heading(...)` calls the `heading` method on the `ui` object.
            ui.heading("My egui Application");

            // `ui.horizontal(|ui| { ... });` uses another closure for horizontal layout.
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                // `ui.text_edit_singleline(&mut self.label);`
                //   `&mut self.label`: Provides a *mutable reference* to the `label` field of our `MyApp` instance (`self`).
                //   This allows the `text_edit_singleline` widget to *directly modify* the `label` field in our state
                //   when the user types into the text box. This is fundamental to egui's state handling.
                ui.text_edit_singleline(&mut self.label);
            });

            // `ui.add(...)` is a general method to add any widget.
            // `egui::Slider::new(&mut self.value, 0.0..=10.0)` creates a slider widget configuration.
            //   `&mut self.value`: Mutably borrows the `value` field from `MyApp`.
            //   `0.0..=10.0`: Defines the range (inclusive) for the slider using Rust's range syntax.
            // `.text("value")`: A builder method to add a label next to the slider.
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            // `if ui.button("Increment").clicked() { ... }`
            //   `ui.button("Increment")`: Creates a button widget and returns a `Response` struct.
            //   `.clicked()`: Calls the `clicked` method on the `Response`. It returns `true` if the button was clicked in this frame, `false` otherwise.
            //   `if ... { ... }`: If `clicked()` is true, execute the code block.
            if ui.button("Increment").clicked() {
                // `self.value += 1.0;`
                //   Accesses the `value` field of our `MyApp` instance (`self`) and increases it by 1.0.
                //   Because `update` has `&mut self`, we are allowed to modify the fields.
                self.value += 1.0;
            }

            // `ui.label(format!(...));` Adds a label.
            // `format!("Hello '{}', value: {}", self.label, self.value)`: A macro to create a formatted `String`.
            //   `{}` are placeholders. `self.label` and `self.value` provide the values to insert.
            //   It reads the *current* state of `self.label` and `self.value` for display.
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }
}

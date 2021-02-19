pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  // vecter with a type of a trait object
  // it's a stand-in for any type inside a Box that implements the Draw trait.
  pub components: Vec<Box<dyn Draw>>, // homogeneous collections,
}

// A generic type parameter can 'only be substituted with one concrete type at a time', whereas
// trait objects 'allow for multiple concrete types' to fill in for the trait object at runtime.
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
  pub background_color: String,
}

impl Draw for Button {
  fn draw(&self) {
    // code to actually draw a button
    println!("drawing button.");
  }
}

impl Button {
  fn on_click(&self) {
    // and define other methods specifically for button
  }
}

pub fn start_gui_app() {
  let screen = Screen {
    components: vec![
      Box::new(Button {
        width: 100,
        height: 100,
        label: String::from("Submit"),
        background_color: String::from("Blue"),
      }),
      Box::new(Button {
        width: 100,
        height: 100,
        label: String::from("Cancel"),
        background_color: String::from("Gray"),
      }),
    ],
  };

  screen.run();
}

// When we use trait objects, Rust must use 'dynamic dispatch'.
// The compiler doesn’t know all the types that might be used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call.
// Instead, at runtime, Rust uses 'the pointers inside the trait object to know which method to call'. 'There is a runtime cost' when this lookup happens that doesn’t occur with static dispatch.
//  Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations

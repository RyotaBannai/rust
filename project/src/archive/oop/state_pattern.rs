// ** encoding state into the type system is also good solution **

// there are three states:
// draft, waiting for review, or published
// Changing from one state to another will be managed internally within the Post type
// The states change in response to the methods called by the library’s users on the Post instance,
// but they don’t have to manage the state changes directly. Also, users can’t make a mistake with the states, like publishing a post before it’s reviewed.

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
    // &self.content

    // 上のように直接返すようにして、published の時だけ返すようにもできるが、
    // all rules in state に則り state の状態で挙動を変えるようにする
    self.state.as_ref().unwrap().content(self) // unwrap() Option
  }

  // どの state の状態で呼んでいるかはわからないが、現在の state は一意なので
  // その state のメソッドが呼ばれる ducking typing のような実装
  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review());
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

// state を変えられるのは State trait のみ
// State trait を呼び出すのが、Post struct
trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self // pending review じゃない状態で approve はできない
  }
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {}) // pending review 状態に戻す
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}
// Because the goal is to 'keep all these rules inside the structs that implement State', we call a content method on the value in state and pass the post instance (that is, self) as an argument. Then we return the value that is returned from using the content method on the state value.

// We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value. Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.

// At this point, when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.

pub fn test() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}

// More features considered:
// ・Add a reject method that changes the post’s state from PendingReview back to Draft.
// ・Require two calls to approve before the state can be changed to Published.
// ・Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

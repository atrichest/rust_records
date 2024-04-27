use std::fmt::Display;

//Defining a Trait
pub trait Summary {
    //A Summary trait that consists of the behavior provided by a summarize method
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    //default implementations
    fn summarize_imple(&self) -> String {
        //Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementatin
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//Implementing a Trait on a Type
//Implementing the Summary trait on the NewsArticle and Tweet types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    //override default implementations
    fn summarize_imple(&self) -> String {
        String::from("(Override Default Implementations...)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

//Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(_item1: &impl Summary, _item2: &impl Summary) {}

pub fn notify4<T: Summary>(_item1: &T, _item2: &T) {}

//Specifying Multiple Trait Bounds with the + Syntax
pub fn notify5(_item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(_item: &T) {}

pub fn notify7<T: Summary + Display, U: Summary>(_t: &T, _u: &U) -> i32 {
    return 1;
}

//Clearer Trait Bounds with where Clauses
pub fn notify8<T, U>(_t: &T, _u: &U) -> i32
where
    T: Summary + Display,
    U: Summary,
{
    return 2;
}

//Returning Types that Implement Traits
//We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know,people"),
        reply: false,
        retweet: false,
    }
}

//Returning either a NewsArticle or a Tweet isn’t allowed
//due to restrictions around how the impl Trait syntax is implemented in the compiler.
//We’ll cover how to write a function with this behavior
//in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.
//```
//fn return_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Oittsburgh,PA,USA"),
//            author: String::from("Iceburgh"),
//            content: String::from(
//                "The Pittsburgh Penguins once again are the est hockey team in the NHL.",
//            ),
//        }
//    } else {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course,as you probably already know,people"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}
//```

//Using Trait Bounds to Conditionally Implement Methods
//Conditionally implementing methods on a generic type depending on trait bounds
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x ={}", self.x);
        } else {
            println!("The largest number is y={}", self.y);
        }
    }
}

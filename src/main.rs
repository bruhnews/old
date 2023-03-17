use serde::Serialize;
use serde_json::{json, Value};
use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://bruh.news";

/// Convert tuple array to article array
macro_rules! articles {
    [ $( ( $h: expr, $s: expr, $i: expr $(,)? ) $(,)? )* ] => {
        &[ $( Article { headline: $h, subtitle: $s, image: $i } ),* ]
    };
}

/// List of article headlines
const ARTICLES: &[Article] = articles![
    (
        "How one man BOUGHT Buckingham Palace",
        "This charming individual has BOUGHT the castle of royalty in BRITAIN!",
        "https://images.pexels.com/photos/3960662/pexels-photo-3960662.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",
    ),
    (
        "NASA reports largest object discovered since JUPITER",
        "Discovered in space; What could it be?",
        "https://images.pexels.com/photos/39561/solar-flare-sun-eruption-energy-39561.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",
    ),
    (
        "Former FDA Scientist Explains Why JOKER Is In Office",
        "Joker (from Joker (2019)) is now President of Czechoslovakia",
        "https://images.pexels.com/photos/2970497/pexels-photo-2970497.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",
    )
];

/// Basic article headline
#[derive(Serialize)]
struct Article {
    headline: &'static str,
    subtitle: &'static str,
    image: &'static str,
}

fn main() -> Result<(), Box<UnreactError>> {
    let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

    app
        // Index page
        .index("index", &json!({ "articles": ARTICLES }))?
        // 404 page
        .not_found("404", &Value::Null)?;

    // Complete app
    app.finish()?;

    Ok(())
}

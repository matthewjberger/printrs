use async_std::println;
use colored::Colorize;

pub async fn howdy() {
    println!("{}", "Howdy!".blue()).await;
}

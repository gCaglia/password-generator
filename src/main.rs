#[cfg(test)]
mod test;

mod randomizer;

use crate::randomizer::create_password;

fn main() {
    let default_size: usize = 30;
    let password = create_password(default_size);
    println!("Your password: {}", password);
}

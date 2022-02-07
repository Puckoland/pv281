use std::fmt::Debug;
use std::io::{stdin, stdout, Result, Write};
use strum::IntoEnumIterator;

pub fn read_line() -> Result<String> {
    stdout().flush()?;

    let mut buf: String = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

pub fn read_number() -> u32 {
    match read_line() {
        Ok(line) => match line.parse::<u32>() {
            Ok(n) => n,
            _ => {
                println!("Invalid number!");
                read_number()
            }
        },
        _ => read_number(),
    }
}

pub fn read_enum<E, I>() -> E
where
    E: IntoEnumIterator<Iterator = I> + Debug,
    I: Iterator<Item = E>,
{
    let enum_full_name = std::any::type_name::<E>();
    let enum_name = match enum_full_name.rfind(':') {
        Some(index) => enum_full_name[index + 1..].to_lowercase(),
        None => enum_full_name.to_lowercase(),
    };
    println!("Select {} type:", enum_name);
    E::iter().enumerate().for_each(|e| println!("{:?}", e));

    let n: usize = read_number() as usize;
    match E::iter().nth(n) {
        Some(item) => item,
        None => {
            println!("Invalid {}! Try again!", enum_name);
            read_enum::<E, I>()
        }
    }
}

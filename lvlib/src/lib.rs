#[no_mangle]
pub extern "C" fn example() -> u16 {
    10
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

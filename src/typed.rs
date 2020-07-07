pub fn get_type_name<T>(it_is: &T) -> String
where
    T: std::fmt::Debug,
{
    let nm = format!("{:?}", it_is);
    nm.split_whitespace().next().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::get_type_name;

    #[derive(Debug)]
    struct Zed();

    #[derive(Debug)]
    struct SuperZed {
        my_name: String,
    }

    #[test]
    fn test_get_type_name() {
        let z = Zed();
        assert_eq!("Zed", get_type_name(&z));

        let sz = SuperZed { my_name: "My name is Zed!".to_owned() };
        assert_eq!("SuperZed", get_type_name(&sz));
    }
}

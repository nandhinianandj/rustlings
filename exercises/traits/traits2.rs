// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!


trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        //self.iter().map(|val| {format!("{}Bar", val.as_str())}).collect()
        //TODO: Ughh. this is another that makes no sense I expect having to append Bar to every
        //String in the vector to add "Bar" as another string in the vector.
        self.push(String::from("Bar"));
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

}

fn main() {
    println!("Write a function to reverse the string");
}

fn get_reverse(input: &str) -> String {
    let coll = input.chars();
    let mut result = String::new();
    for i in coll.rev() {
      result.push(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reverse(){
        assert_eq!(String::from("racecar"), get_reverse("racecar"));
        assert_eq!(String::from("desserts"), get_reverse("stressed"));
        assert_eq!(String::from("strops"), get_reverse("sports"));
    }
}
#[doc = include_str!("../README.md")]

/// Say hi to someone! <3
pub fn greet(name: &str) -> String {
	format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
	use crate::greet;

	#[test]
	fn greet_bob() {
		assert_eq!(greet("Bob"), String::from("Hello Bob!"));
	}

	#[test]
	fn greet_the_world() {
		assert_eq!(greet("World"), String::from("Hello World!"));
	}
}

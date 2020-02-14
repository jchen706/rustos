// FIXME: Make me pass! Diff budget: 30 lines.



struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

<<<<<<< HEAD
impl Builder {
    fn default()-> Builder { Builder {
	string: None,
	number: None,
    }}
    fn string<T:Into<String>>(&mut self, x: T)-> &mut Self {
	self.string = Some(x.into());
	self
    }
    fn number(&mut self, x:usize)->&mut Self{
	self.number = Some(x);
	self
    }
}
impl ToString for Builder {
    // Implement the trait
    fn to_string(&self)->String{
	if self.number.is_some() && self.string.is_some() {
		[self.string.clone().unwrap() ,self.number.unwrap().to_string()].join(" ")
	} else if self.number.is_some() {
		self.number.unwrap().to_string()
	} else if self.string.is_some() {		
		self.string.clone().unwrap()
	} else {
		String::from("")
	}	
    }
}

=======
>>>>>>> skeleton/lab2
// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}

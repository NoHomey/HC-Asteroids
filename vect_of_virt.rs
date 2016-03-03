trait Person {
    fn get(&self) -> i32;
}

struct Ivo { ivo: i32 }

impl Person for Ivo {
	fn get(&self) -> i32 {
		self.ivo * 3
	}
}

struct Bobi { bobi: i32 }

impl Person for Bobi {
	fn get(&self) -> i32 {
		self.bobi * 2
	}
}

impl Ivo {
	fn new(i: i32) -> Ivo {
		Ivo {ivo: i}
	}
}

impl Bobi {
	fn new(i: i32) -> Bobi {
		Bobi {bobi: i}
	}
}

fn main() {
    let ivo: Ivo = Ivo::new(5);
    let bobi: Bobi = Bobi::new(3);
    let mut v: Vec<Box<Person>> = Vec::new();
    v.push(Box::new(ivo));
    v.push(Box::new(bobi));
    for person in v.iter() {
	println!("{}", person.get());	
    }	
}

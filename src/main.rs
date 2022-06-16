// Struct with implementations + trait
struct Place {
    x: i32,
    y: i32,
    z: i32
}

impl Default for Place {
    fn default() -> Self {
        Place { 
            x: 1, 
            y: 0, 
            z: 3 
        }
    }
}

trait HighestValues {
    fn findhighest(self: &Self) -> i32;
}

impl HighestValues for Place {
    fn findhighest(self: &Self) -> i32 {

        if self.x > self.y && self.x > self.z {
            self.x
        }
        else if self.y > self.x && self.y > self.z {
            self.y
        }
        else {
            self.z
        }
    }
    
}
// ----------------------------------


// Enumerations

enum School {
    Student,
    Teacher,
    Guest,
    Undecided
}

fn school_identifier(person: School) {
    match person {
        School::Student => {
            println!("This is a student")
        }
        School::Teacher => {
            println!("This is a teacher")
        }
        School::Guest => {
            println!("This is a guest")
        }
        School::Undecided => {
            println!("This is a undecided person")
        }
    }
}

// ----------------------------------


fn main() {
    print!("Playground starting!");
    println!("Ok!");
    
    // Struct
    let pos = Place{y: 2, ..Default::default()};

    println!("Postion: {}, {}, {}", pos.x, pos.y, pos.z);
    println!("Highest value: {}", pos.findhighest());
    println!("Postion: {}, {}, {}", pos.x, pos.y, pos.z);


    // Enum
    let boy: School = School::Student;
    school_identifier(boy);
    let lady: School = School::Teacher;
    school_identifier(lady);

} 

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    let place_test = Place{..Default::default()};
    assert_eq!(place_test, Place{x: 1, y: 0, z: 3});
} */
/*
    Made for learning rust
*/

use std::io::Error;


// ----- Struct with implementations + trait -----
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
// ---------------------------------------


// ------------ Enumerations -------------

enum School {
    Student,
    Teacher,
    Guest,
    Undecided
}

fn school_identifier(person: &School) {
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

// ------------------------------------


// --------- Vector/function ----------

fn make_vector_add_content() -> Vec<i32> {
    let my_vec = vec![1, 2, 3];
    my_vec
}

fn double_over_vec(iter_vec: &mut Vec<i32>) -> &Vec<i32> {
    let vec_len = iter_vec.len();
    for i in 0..vec_len {
        iter_vec[i] *= 2;
    }
    iter_vec 
}


// ----------------------------------



// --------- Error handling ----------

fn great_guest(person: &School) -> Result<String, String>{
    match person {
        School::Guest => {
            Ok("Hello guest!".to_string())
        }
        _ => {
            Err("Not a guest!".to_string())
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
    school_identifier(&boy);
    let lady: School = School::Teacher;
    school_identifier(&lady);


    // Vector
    let mut generated_vector = make_vector_add_content();
    println!("First element {}",&generated_vector[0]);
    double_over_vec(&mut generated_vector);
    println!("First element after double_over_vec {}",&generated_vector[0]);

    // Error handling
    println!("{:?}", great_guest(&lady));
    let random_pers: School = School::Guest;
    println!("{:?}", great_guest(&random_pers));
    

} 


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn default_struct() {
        let place_test = Place{..Default::default()};
        //let place_default = Place{x: 1, y: 0, z: 3};
        assert_eq!(place_test.x, 1);
        assert_eq!(place_test.y, 0);
        assert_eq!(place_test.z, 3);
    }

    #[test]
    fn place_struct_values() {
        let place_point = Place{x: 2, y: 9, z: 4};
        assert_eq!(place_point.x, 2);
        assert_eq!(place_point.y, 9);
        assert_eq!(place_point.z, 4);
    }

    #[test]
    fn highest_value_point() {
        let place_test = Place{..Default::default()};
        assert_eq!(place_test.findhighest(), 3)
    }

    #[test]
    fn enum_matches() {
        let guest_person: School = School::Guest;
        match  guest_person {
            School::Guest => {
                ()
            }
            School::Student => {
                panic!()
            }
            School::Teacher => {
                panic!()
            }
            School::Undecided => {
                panic!()
            }
        }

        let guest_person: School = School::Undecided;
        match  guest_person {
            School::Guest => {
                panic!()
            }
            School::Student => {
                panic!()
            }
            School::Teacher => {
                panic!()
            }
            School::Undecided => {
                ()
            }
        }
    }

    #[test]
    fn double_vec() {
        let mut generated_vector = make_vector_add_content();
        double_over_vec(&mut generated_vector);
        assert_eq!(generated_vector[0], 2)
    }

    #[test]
    fn error_guest_true() {
        let random_pers: School = School::Guest;
        assert_eq!(great_guest(&random_pers), Ok("Hello guest!".to_string()))
    }

    #[test]
    fn error_guest_false() {
        let random_pers: School = School::Teacher;
        assert_eq!(great_guest(&random_pers), Err("Not a guest!".to_string()))
    }

}
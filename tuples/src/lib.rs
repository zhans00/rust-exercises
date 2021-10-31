use std::fmt;

pub struct Student {
    id: i32,
    fname: String,
    lname: String,
}


impl fmt::Debug for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Student({}, \"{}\", \"{}\")", self.id, self.fname, self.lname)
    }
}


pub fn Student(id: i32, fname: String, lname: String) -> Student {
    return Student{
        id : id,
        fname :fname,
        lname : lname,
    };

}

pub fn id(student: &Student) -> i32 {
    return student.id;
}

pub fn first_name(student: &Student) -> String {
    return student.fname.to_string();
}

pub fn last_name(student: &Student) -> String {
    return student.lname.to_string();
}

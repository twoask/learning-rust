use chrono::NaiveDate;

enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

struct Student {
    name: String,
    student_id: String,
    dob: NaiveDate, // proper date type
    current_college: String,
    start_date: NaiveDate,
    graduation_date: NaiveDate,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        addr: String::from("::1"),
    };

    // Example of creating a Student with proper date types
    let student = Student {
        name: String::from("John Doe"),
        student_id: String::from("12345"),
        dob: NaiveDate::from_ymd_opt(2000, 5, 15).unwrap(), // May 15, 2000
        current_college: String::from("Rust University"),
        start_date: NaiveDate::from_ymd_opt(2020, 8, 30).unwrap(), // Aug 30, 2020
        graduation_date: NaiveDate::from_ymd_opt(2024, 5, 20).unwrap(), // May 20, 2024
    };

    println!("Student: {}", student.name);
    println!("Date of Birth: {}", student.dob.format("%m/%d/%Y"));
    println!("Start Date: {}", student.start_date.format("%m/%d/%Y"));
    println!("Graduation Date: {}", student.graduation_date.format("%m/%d/%Y"));
}

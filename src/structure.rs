pub fn struc() {
    struct Employee {
        employee_name: String,
        employee_id: u64,
        employee_profile: String,
        active: bool,
    }

    let mut employee1 = Employee {
        employee_name: String::from("Pankaj Chaudhary "),
        employee_id: 1307,
        employee_profile: String::from("Software Consultant "),
        active: true,
    };

    println!("{ } ", employee1.employee_name);

    employee1.employee_name = String :: from("Deepak Chaudhary");

    println!("Enter working days of { } ", employee1.employee_name);


    struct moodule{
        employee_name: String,
        employee_id: u64,
        employee_eff: u64,
        employee_working_day: u64,

    }

    let mut moodule1=moodule {
        employee_name: employee1.employee_name,
        employee_id: 1307,
        employee_eff: 3,
        employee_working_day: 0,

    };

    let i:i32  = read!();
    moodule1.employee_working_day= i as u64;

    let p= (module_del(moodule1.employee_eff as i32, moodule1.employee_working_day as i32));

    print!("Total Module Delivered by {} is : {}" , moodule1.employee_name, p);


}
pub fn module_del(a :i32 , b :i32 )-> i32 {
    a*b
}
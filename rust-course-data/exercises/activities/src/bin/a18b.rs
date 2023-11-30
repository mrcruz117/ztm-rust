// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Assembly,
}
struct Employee {
    name: String,
    employee_type: EmployeeType,
    active: bool,
}

fn access_building(employee: &Employee) -> Result<(), String> {
    if !employee.active {
        return Err(String::from("not active employee"));
    }
    match employee.employee_type {
        EmployeeType::Maintenance => Ok(()),
        EmployeeType::Marketing => Ok(()),
        EmployeeType::Manager => Ok(()),
        _ => Err(String::from("Access denied")),
    }
}

fn main() {
    let emp = Employee {
        name: String::from("John"),
        employee_type: EmployeeType::Maintenance,
        active: false,
    };
    let result = access_building(&emp);
    match result {
        Ok(()) => println!("Access granted"),
        Err(err) => println!("Access denied: {}", err),
    }

}

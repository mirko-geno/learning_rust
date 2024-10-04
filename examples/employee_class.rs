struct Employee {
     name: String,
     age: u8,
     salary: f32,
     balance: f32
}

impl Employee{
    fn pay(&mut self){
        self.balance += self.salary;
    }
}

fn main(){
    let mut employees: Vec<Employee> = Vec::new();
    employees.push(Employee{name: String::from("Mirko"), age:17, salary:891285.0, balance:0.0});
    employees.push(Employee{name: String::from("Leo"), age:17, salary:402589.0, balance:0.0});
    employees.push(Employee{name: String::from("Tiyu"), age:17, salary:28513.0, balance:0.0});

    employees[0].pay();

    for emp in employees{
        println!("Employee {}, age:{}, salary:{}, balance:{}", emp.name, emp.age, emp.salary, emp.balance);
    }    
}

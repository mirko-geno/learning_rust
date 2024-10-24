struct Employee {
    name: String,
    age: u8,
    salary: f32,
    balance: f32
}

impl Employee{
   fn new(name: String, age: u8, salary: f32) -> Employee{
       Employee{name, age, salary, balance: 0.0}
   }

   fn pay(&mut self){
       self.balance += self.salary;
   }
}

fn main(){
   let mut employees: Vec<Employee> = Vec::new();
   employees.push(Employee::new(String::from("Mirko"), 17, 891285.0));
   employees.push(Employee::new(String::from("Leo"), 17, 402589.0));
   employees.push(Employee::new(String::from("Tiyu"), 17, 28513.0));

   employees[0].pay();

   for emp in employees{
       println!("Employee {}, age:{}, salary:{}, balance:{}", emp.name, emp.age, emp.salary, emp.balance);
   }    
}

mod mall;

pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) ->  (&String, &Store) {
    mall.floors
        .values()
        .map(|f| f.stores.iter())
        .flatten()
        .max_by_key(|(_, s)| s.square_meters)
        .unwrap()
        
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut max_salary = f64::MIN;
    let mut highest_paid = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push((name, employee));
                } else if (employee.salary - max_salary).abs() < 0.01 {
                    highest_paid.push((name, employee));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .values()
            .map(|f| f.stores.values())
            .flatten()
            .map(|store| store.employees.len())
            .sum::<usize>()
}

pub fn check_for_securities(mall: &mut Mall, available_sec: HashMap<String, Guard>) {
    let total_size: u64 = mall.floors.values().map(|floor| floor.size_limit).sum();
    let required_guards = (total_size / 200) as usize;
    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        let needed = required_guards - current_guards;
        for (name, guard) in available_sec.into_iter().take(needed) {
            mall.hire_guard(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1.saturating_sub(employee.working_hours.0);
                let adjustment = employee.salary * 0.1;
                if hours >= 10 {
                    employee.raise(adjustment);
                } else {
                    employee.cut(adjustment);
                }
            }
        }
    }
}

pub mod mall;

pub use floor::store;
pub use mall::*;
pub use store::employee;

pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut res: store::Store = store::Store::new("", 0, vec![]);

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            if shop.square_meters > res.square_meters {
                res = shop.clone();
            }
        }
    }

    res
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut res = vec![employee::Employee::new("", 0, 0, 0, 0.0)];

    for elem in mall.floors.iter() {
        for shop in elem.stores.iter() {
            for emp in shop.employees.clone().into_iter() {
                if emp.salary > res[0].salary {
                    res[0] = emp.clone();
                } else if emp.salary == res[0].salary {
                    res.push(emp.clone());
                }
            }
        }
    }

    res
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut res = 0;

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            res += shop.employees.len();
        }
    }

    res + mall.guards.len()
}

pub fn check_for_securities(mall: &mut mall::Mall, available_sec: Vec<guard::Guard>) {
    let mut size = 0;

    for floor in mall.floors.iter() {
        size += floor.size_limit;
    }

    let mut i = 0;
    while (mall.guards.len() as f64) < size as f64 / 200.0 {
        mall.hire_guard(available_sec[i].clone());
        i += 1;
    }
}

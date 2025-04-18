#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut worker) = self.grade.take() {
            self.grade = worker.next.take(); // move to the next worker
            Some(worker.name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        if let Some(worker) = &self.grade {
            Some((worker.name.clone(), worker.role.clone()))
        } else {
            None
        }
    }
}

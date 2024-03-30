use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending {super_struct: base}
    }
}

#[cfg(test)]
mod pending_tests {
    use super::Pending;
    use super::TaskStatus;

    #[test]
    fn new() {
        let expected_title = String::from("test title");
        let expected_status = TaskStatus::PENDING;
        let new_pending_struct = Pending::new(&expected_title);

        assert_eq!(expected_title, new_pending_struct.super_struct.title);
        assert_eq!(expected_status, new_pending_struct.super_struct.status);
    }
}

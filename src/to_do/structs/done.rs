use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done {super_struct: base}
    }
}

#[cfg(test)]
mod done_tests {
    use super::Done;
    use super::TaskStatus;

    #[test]
    fn new() {
        let expected_title = String::from("test title");
        let expected_status = TaskStatus::DONE;
        let new_done_struct = Done::new(&expected_title);

        assert_eq!(expected_title, new_done_struct.super_struct.title);
        assert_eq!(expected_status, new_done_struct.super_struct.status);
    }
}

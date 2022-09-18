use crate::ided::Ided;

pub struct Student {
    pub student_id: u64,
}

impl Ided for Student {
    fn my_id(&self) -> u64 {
        self.student_id
    }
}

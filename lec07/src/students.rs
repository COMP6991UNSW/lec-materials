//! ...and some more documentation within students.rs!!

pub mod utils;

pub struct Student {
    pub(crate) name: String,
    pub zid: u32,
    wam: Option<f64>,
}

pub(super) fn some_student_fn() {
    crate::students::utils::utility();
    utils::utility();
}

impl Student {
    pub fn new(name: String, zid: u32) -> Student {
        some_student_fn();

        Student {
            name,
            zid,
            wam: None,
        }
    }

    pub fn wam(&self) -> Option<f64> {
        self.wam
    }

    pub fn set_wam(&mut self, wam: f64) {
        if wam >= 0.0 && wam <= 100.0 {
            self.wam = Some(wam);
        }
    }
}

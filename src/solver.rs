use crate::*;

lazy_static! {
    pub static ref SOLVER: Solver = Solver::new();
}

pub struct Solver {
    pointer: *mut std::ffi::c_void,
    literals: RefCell<u32>,
    clauses: RefCell<u32>,
}

impl Solver {
    pub fn new() -> Self {
      Self {
          pointer: unsafe { ipasir_init() },
          literals: RefCell::new(0),
          clauses: RefCell::new(0),
      }
    }

    pub fn add(&self, literal: i32) {
        unsafe { ipasir_add(self.pointer, literal); }
        if literal == 0 { *self.clauses.borrow_mut() += 1; }
    }

    pub fn run(&self) -> bool {
        let status = unsafe { ipasir_solve(self.pointer) };
        match status { 10 => true, 20 => false, _ => panic!() }
    }

    pub fn new_literal(&self) -> i32 {
        *self.literals.borrow_mut() += 1;
        *self.literals.borrow() as i32
    }

    pub fn assignment(&self, literal: i32) -> bool {
        literal == unsafe { ipasir_val(self.pointer, literal) }
    }

    pub fn literals(&self) -> u32 {
        *self.literals.borrow()
    }

    pub fn clauses(&self) -> u32 {
        *self.clauses.borrow()
    }
}

unsafe impl Send for Solver { }
unsafe impl Sync for Solver { }

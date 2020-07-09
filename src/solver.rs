use crate::*;

lazy_static! {
    pub static ref SOLVER: Solver = Solver::new();
}

pub struct Solver {
    literals: RefCell<u32>,
    clauses: RefCell<Vec<Vec<i32>>>,
    universals: RefCell<Vec<i32>>,
    assignments: RefCell<HashMap<i32, bool>>,
}

impl Solver {
    pub fn new() -> Self {
      Self {
          literals: RefCell::new(0),
          clauses: RefCell::new(vec![vec![]]),
          universals: RefCell::new(vec![]),
          assignments: RefCell::new(HashMap::new()),
      }
    }

    pub fn add(&self, literal: i32) {
        let mut clauses = self.clauses.borrow_mut();

        if literal == 0 {
            clauses.push(vec![]);
        } else {
            clauses.last_mut().unwrap().push(literal);
        }
    }

    pub fn universal(&self, literal: i32) {
        self.universals.borrow_mut().push(literal);
    }

    pub fn new_literal(&self) -> i32 {
        *self.literals.borrow_mut() += 1;
        *self.literals.borrow() as i32
    }

    pub fn literals(&self) -> u32 {
        *self.literals.borrow()
    }

    pub fn clauses(&self) -> u32 {
        self.clauses.borrow().len() as u32 - 1
    }

    pub fn qdimacs(&self) -> String {
        let mut s = String::new();

        writeln!(&mut s, "p cnf {} {}", self.literals(), self.clauses()).unwrap();

        let universals = self.universals.borrow().iter().map(|u| u.to_string()).collect::<Vec<_>>();
        if !universals.is_empty() {
            writeln!(&mut s, "a {} 0", universals.join(" ")).unwrap();
        }

        for clause in self.clauses.borrow().iter() {
            if clause.is_empty() { break; }

            let literals = clause.iter().map(|l| l.to_string()).collect::<Vec<_>>();
            writeln!(&mut s, "{} 0", literals.join(" ")).unwrap();
        }

        s
    }

    pub fn run(&self) -> bool {
        let mut file = File::create("target/tmp.qdimacs").unwrap();
        file.write_all(self.qdimacs().as_bytes()).unwrap();

        let output = Command::new("caqe")
            .arg("--qdo")
            .arg("target/tmp.qdimacs")
            .output().unwrap();

        let stdout = from_utf8(&output.stdout).unwrap();
        let stderr = from_utf8(&output.stderr).unwrap();

        if !stderr.is_empty() {
            println!("{}", stderr);
        }

        self.parse_assignments(stdout);

        stdout.lines().last().unwrap() == "c Satisfiable"
    }

    pub fn assignment(&self, literal: i32) -> bool {
        *self.assignments.borrow().get(&literal).unwrap()
    }

    fn parse_assignments(&self, stdout: &str) {
        let mut assignments = self.assignments.borrow_mut();

        assignments.clear();

        for line in stdout.to_string().lines() {
            let split = line.split(" ").collect::<Vec<_>>();
            if split.len() != 3 { continue; }

            let literal = split[1].parse::<i32>().unwrap();

            assignments.insert(literal, true);
            assignments.insert(-literal, false);
        }
    }
}

unsafe impl Send for Solver { }
unsafe impl Sync for Solver { }

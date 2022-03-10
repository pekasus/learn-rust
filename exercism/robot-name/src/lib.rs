use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    static USED_NAMES: RefCell<HashSet<String>>= RefCell::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let name = Self::generate_name();
        Robot { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_name();
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        loop {
            let mut vec: Vec<char> = (0..2)
                .map(|_| rng.gen_range(b'A'..b'Z' + 1) as char)
                .collect();
            vec.extend((0..3).map(|_| rng.gen_range(b'0'..b'9') as char));
            let name = vec.iter().collect::<String>();

            if USED_NAMES.with(|names| names.borrow_mut().insert(name.clone())) {
                return name;
            }
        }
    }
}

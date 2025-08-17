use std::collections::HashMap;
use std::rc::Rc;

pub struct GcdCache {
    data: HashMap<(i128, i128), i128>
}


fn eucleidian_gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

impl GcdCache {

    pub fn init() -> Rc<GcdCache> {
        Rc::new(GcdCache {data: HashMap::new()})
    }


    pub fn gcd(&mut self, a: i128, b: i128) -> i128 {
        let (bigger, smaller) = if (a > b) {(a, b)} else {(b, a)};

        match self.data.get(&(bigger, smaller)) {
            Some(res) => {return *res},
            None => {
                let gcd = eucleidian_gcd(bigger, smaller);
                self.data.insert((bigger, smaller), gcd);
                return gcd;
            }
        }
    }

}


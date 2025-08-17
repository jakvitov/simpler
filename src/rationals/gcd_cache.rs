use std::collections::HashMap;
use std::rc::Rc;

use super::numerical_error::NumericalError;

pub struct GcdCache {
    data: HashMap<(i128, i128), i128>
}

//Gcd implemented using the Eucleidian algorithm
fn gcd_eucleidian(mut a: i128, mut b: i128) -> Result<i128, Box<NumericalError>> {
    if (a == b && b == 0) {
        return Result::Err(Box::new(NumericalError::new("Both arguments of gcd are zero.", format!("gcd({}, {})", a,b))));
    } else if (a < 0 || b < 0) {
        return Result::Err(Box::new(NumericalError::new("Some of the gcd arguments are smaller than zero", format!("lcm({},{})", a,b))));
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return Result::Ok(a);
}

fn lcm_eucleidian(a: i128, b: i128, gcd_cache: &mut GcdCache) -> Result<i128, Box<NumericalError>> {
    if (a == b && b == 0) {
        return Result::Err(Box::new(NumericalError::new("Both arguments of LCM are zero.", format!("lcm({},{})", a,b))));
    }
    let gcd = gcd_cache.gcd(a,b)?;
    return Result::Ok(((a*b) /gcd));
}

impl GcdCache {

    pub fn init() -> GcdCache {
        GcdCache {data: HashMap::new()}
    }


    pub fn gcd(&mut self, a: i128, b: i128) -> Result<i128, Box<NumericalError>> {
        let (bigger, smaller) = if (a > b) {(a, b)} else {(b, a)};

        return match self.data.get(&(bigger, smaller)) {
            Some(res) => Result::Ok({*res} ),
            None => {
                let gcd = gcd_eucleidian(bigger, smaller)?;
                self.data.insert((bigger, smaller), gcd );
                return Result::Ok(gcd);
            }
        }
    }

    pub fn lcm(&mut self, a: i128, b: i128) -> Result<i128, Box<NumericalError>> {
        let (bigger, smaller) = if (a > b) {(a, b)} else {(b, a)};
        let res = lcm_eucleidian(a, b, self)?;
        return Result::Ok(res);
    }

}

#[cfg(test)]
mod tests {
    use crate::rationals::gcd_cache::GcdCache;

    #[test]
    fn gcd_works_correctly_for_non_zero_parameters() {
        let mut cache: GcdCache = GcdCache::init();
        let a = 550;
        let b = 10;
        let res = cache.gcd(a,b);
        assert!(res.is_err() == false);

        let Ok(result) = res else {
            panic!("Result is err");
        };

        assert_eq!(result, 10);
    }




}
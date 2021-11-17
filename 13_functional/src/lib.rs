use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// The Cacher struct has a calculation field of the generic type T. The trait bounds on T specify
// that it’s a closure by using the Fn trait. Any closure we want to store in the calculation
// field must have one u32 parameter (specified within the parentheses after Fn) and must
// return a u32 (specified after the ->).
struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Eq + Hash + Copy,
        V: Copy,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run fo {} minutes", expensive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cacher;

    #[test]
    fn call_cacher_with_different_values() {
        let mut c = Cacher::new(|a| a);

        assert_eq!(1, c.value(1));
        assert_eq!(2, c.value(2));
    }

    #[test]
    fn call_cacher_with_string_slice() {
        let mut c = Cacher::new(|a: &str| a);

        assert_eq!("hello", c.value("hello"));
    }

    #[test]
    fn call_cacher_with_usize() {
        let mut c = Cacher::new(|a: usize| a);

        assert_eq!(500, c.value(500));
    }
}

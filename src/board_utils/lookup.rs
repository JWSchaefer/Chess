

pub struct Lookup<T, U> {
    pub keys : Vec<T>,
    pub values : Vec<U>
}


impl<T , U> Lookup<T , U> where
T : Eq,
U : Eq 
{

    pub fn get_value(&self, key : &T) -> Result<&U,  &'static str>{
        match self.keys.iter().position(|i : &T| i == key) {
            Some(index) => {Ok(&self.values[index])},
            None => {Err("Key not found in lookup")}
        }
    }

    pub fn get_key(&self, value : &U) -> Result<&T,  &'static str>{
        match self.values.iter().position(|i : &U| i == value) {
            Some(index) => {Ok(&self.keys[index])},
            None => {Err("Value not found in lookup")}
        }
    }

}
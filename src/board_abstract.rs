pub trait Board {

    fn new() -> Self;

    fn force_valid(&self);
    fn check_valid(&self) -> bool;
    
}



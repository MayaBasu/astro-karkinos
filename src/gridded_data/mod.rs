pub mod data1d;
pub mod data2d;
pub mod units;

pub use self::data1d::*;
pub use self::data2d::*;
pub use self::units::*;


#[cfg(test)]
mod tests {

    #[test]
    pub fn test(){
        println!("hello??")
    }
    
}

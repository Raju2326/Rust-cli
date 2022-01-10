
use std::fmt::{self,Debug,Result};
pub struct ArgConfig{
   pub search:String,
   pub path:String
}
impl ArgConfig {
    fn new(search:String, path:String)->ArgConfig{
        ArgConfig{
           search,
           path
        }
    }

}
impl Debug for ArgConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArgConfig")
         .field("search", &self.search)
         .field("path", &self.path)
         .finish()
    }
}
    


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

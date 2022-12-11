
    pub struct ToChimpoService {
    }
    
    impl ToChimpoService {
        pub fn new() -> Self {
            ToChimpoService {}
        }
       
        pub fn add_one_to_all(&self, nums: &mut Vec<u8>) {
            for num in nums.iter_mut() {
                *num += 1;
            }
        }
      
        pub fn remove_chimpos<'a>(&'a self, disgusting_words: Vec<&'a str>) -> Vec<&str> {
         disgusting_words
         .into_iter()
         .filter(|s| !s.contains("chimpo"))
         .collect()
        }
      
        pub fn to_chimpos(&self, chinkos: Vec<&str>) -> Vec<String> {
            chinkos
              .into_iter()
              .map(|s| s.replace("chinko", "chimpo"))
              .collect()
        }
    }
    


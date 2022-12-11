mod service;  
use service::to_chinko_service::ToChimpoService;
  
  fn main() {
    let service = ToChimpoService::new();
    let strings = vec!["chinko1", "chinko2", "chinko3"];
    let disgusting_words = vec!["chinko1", "chinko2", "chinko3","chimpo1", "chimpo2", "chimpo3"];
    let clean_words = service.remove_chimpos(disgusting_words);
    let mut nums = vec![8,7, 6];
    service.add_one_to_all(&mut nums);
    let chimpos = service.to_chimpos(strings);
  
    assert_eq!(bytes, vec![9,8, 7]);
    assert_eq!(chimpos, vec!["chimpo1", "chimpo2", "chimpo3"]);
    assert_eq!(clean_words, vec!["chinko1", "chinko2", "chinko3"]);
  }

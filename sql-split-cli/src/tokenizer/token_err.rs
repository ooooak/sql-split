
#[derive(Debug)]
pub struct TokenErr{
  // line_num: usize,
  pub text: &'static str
}



pub fn terr<T>(text: &'static str) -> Result<T, TokenErr> {
    Err(TokenErr{ text })
}
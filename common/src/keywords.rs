use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
  Fn,
  Return,
  If,
  Else,
  Match,
  Const,
  Let,
  For,
  While,
  Do,
  Break,
  Continue,
  Primitive(Primitive),
}

#[derive(Copy, Clone, Debug)]
pub enum Primitive {
  U8,
  U16,
  U32,
  U64,
  I8,
  I16,
  I32,
  I64,
  F32,
  F64,
  Boolean,
}

impl TryFrom<&str> for Keyword {
  type Error = ();

  fn try_from(keyword: &str) -> Result<Self, Self::Error> {
    match keyword {
      "fn" => Ok(Keyword::Fn),
      "return" => Ok(Keyword::Return),

      // Types
      "boolean" => Ok(Keyword::Primitive(Primitive::Boolean)),
      "i32" => Ok(Keyword::Primitive(Primitive::I32)),
      "i64" => Ok(Keyword::Primitive(Primitive::I64)),
      "u32" => Ok(Keyword::Primitive(Primitive::U32)),
      "u64" => Ok(Keyword::Primitive(Primitive::U64)),
      "f32" => Ok(Keyword::Primitive(Primitive::F32)),
      "f64" => Ok(Keyword::Primitive(Primitive::F64)),

      "for" => Ok(Keyword::For),
      "do" => Ok(Keyword::Do),
      "while" => Ok(Keyword::While),
      "break" => Ok(Keyword::Break),
      "continue" => Ok(Keyword::Continue),
      "if" => Ok(Keyword::If),
      "else" => Ok(Keyword::Else),
      "match" => Ok(Keyword::Match),

      _ => Err(()),
    }
  }
}

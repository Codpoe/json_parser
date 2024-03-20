use crate::{
  parser::{
    ArrayAst, BoolAst, IdentifierAst, NullAst, NumberAst, ObjectAst, PropertyAst, StringAst,
  },
  Json,
};

pub trait Visit {
  fn visit_json(&mut self, ast: &mut Json) {
    match ast {
      Json::String(ast) => self.visit_string(ast),
      Json::Number(ast) => self.visit_number(ast),
      Json::Boolean(ast) => self.visit_boolean(ast),
      Json::Null(ast) => self.visit_null(ast),
      Json::Object(ast) => self.visit_object(ast),
      Json::Property(ast) => self.visit_property(ast),
      Json::Identifier(ast) => self.visit_identifier(ast),
      Json::Array(ast) => self.visit_array(ast),
    }
  }

  fn visit_string(&mut self, _ast: &mut StringAst) {}

  fn visit_number(&mut self, _ast: &mut NumberAst) {}

  fn visit_boolean(&mut self, _ast: &mut BoolAst) {}

  fn visit_null(&mut self, _ast: &mut NullAst) {}

  fn visit_object(&mut self, ast: &mut ObjectAst) {
    for property in ast.value.iter_mut() {
      self.visit_property(property);
    }
  }

  fn visit_property(&mut self, ast: &mut PropertyAst) {
    self.visit_identifier(&mut ast.key);
    self.visit_property_value(&mut ast.value);
  }

  fn visit_identifier(&mut self, ast: &mut IdentifierAst) {
    self.visit_string(&mut ast.value);
  }

  fn visit_property_value(&mut self, ast: &mut Json) {
    self.visit_json(ast);
  }

  fn visit_array(&mut self, ast: &mut ArrayAst) {
    for item in ast.value.iter_mut() {
      self.visit_array_item(item);
    }
  }

  fn visit_array_item(&mut self, ast: &mut Json) {
    self.visit_json(ast);
  }
}

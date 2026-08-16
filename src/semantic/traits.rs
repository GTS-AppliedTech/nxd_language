use crate::semantic::errors::SemanticError;
use crate::ir::nodes::{IRTrait, IRImpl};

pub struct TraitRegistry {
    pub traits: Vec<IRTrait>,
    pub impls: Vec<IRImpl>,
}

impl TraitRegistry {
    pub fn implements(&self, trait_name: &str, ty: &str) -> bool {
        self.impls.iter().any(|im| im.trait_name == trait_name && im.target_type == ty)
    }

    pub fn check_trait_call(
        &self,
        trait_name: &str,
        ty: &str,
    ) -> Result<(), SemanticError> {
        if self.implements(trait_name, ty) {
            Ok(())
        } else {
            Err(SemanticError::TraitNotImplemented {
                trait_name: trait_name.to_string(),
                ty: ty.to_string(),
            })
        }
    }
}

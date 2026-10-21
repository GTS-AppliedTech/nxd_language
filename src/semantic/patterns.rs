use crate::ir::nodes::IRMatchArm;
use crate::semantic::errors::SemanticError;

pub fn validate_match_arms(arms: &Vec<IRMatchArm>) -> Result<(), SemanticError> {
    // TODO: exhaustiveness checking
    // TODO: variant correctness
    // TODO: type compatibility
    Ok(())
}

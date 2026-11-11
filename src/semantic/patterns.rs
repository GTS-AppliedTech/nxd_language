use crate::diagnostics::errors::SemanticError;
use crate::ir::nodes::IRMatchArm;

pub fn validate_match_arms(_arms: &Vec<IRMatchArm>) -> Result<(), SemanticError> {
    // TODO: exhaustiveness checking
    // TODO: variant correctness
    // TODO: type compatibility
    Ok(())
}

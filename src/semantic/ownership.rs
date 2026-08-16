use crate::semantic::errors::SemanticError;

pub enum OwnershipOp {
    Move,
    Clone,
    Borrow,
}

pub fn check_ownership(op: OwnershipOp, ty: &str) -> Result<(), SemanticError> {
    match op {
        OwnershipOp::Move => Ok(()), // MOVE always allowed
        OwnershipOp::Clone => Ok(()), // TODO: forbid clone on non-cloneable types
        OwnershipOp::Borrow => Ok(()), // BORROW always allowed for now
    }
}

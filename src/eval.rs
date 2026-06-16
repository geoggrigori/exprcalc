//! Evaluator: walks an [`Expr`] tree and computes its `f64` value.

use crate::parser::{BinOp, Expr};
use crate::Error;

/// Evaluate an [`Expr`] AST into an `f64`.
///
/// # Errors
///
/// Returns [`Error::DivisionByZero`] if a division or remainder by zero
/// is attempted.
pub fn eval_expr(expr: &Expr) -> Result<f64, Error> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Neg(inner) => Ok(-eval_expr(inner)?),
        Expr::Binary { op, left, right } => {
            let l = eval_expr(left)?;
            let r = eval_expr(right)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err(Error::DivisionByZero)
                    } else {
                        Ok(l / r)
                    }
                }
                BinOp::Rem => {
                    if r == 0.0 {
                        Err(Error::DivisionByZero)
                    } else {
                        Ok(l % r)
                    }
                }
            }
        }
    }
}

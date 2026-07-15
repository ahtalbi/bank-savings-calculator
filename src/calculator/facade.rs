use crate::calculator::types::BudgetResult;
use crate::calculator::traits::BudgetCalculator;
use crate::calculator::engine::BudgetEngine;

/// facade that provides a simple interface for the budget calculator
pub struct BudgetFacade {
    engine: BudgetEngine,
}

impl BudgetFacade {
    pub fn new() -> Self {
        BudgetFacade {
            engine: BudgetEngine,
        }
    }

    /// calculate budget breakdown for a given total amount
    pub fn calculate(&self, total: f64) -> BudgetResult {
        self.engine.calculate(total)
    }
}
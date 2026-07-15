use crate::calculator::types::BudgetResult;

/// trait for calculating budget breakdown from a total amount
pub trait BudgetCalculator {
    fn calculate(&self, total: f64) -> BudgetResult;
}

/// trait for formatting budget results into display strings
#[allow(dead_code)]
pub trait BudgetFormatter {
    fn format_breakdown(&self, result: &BudgetResult) -> Vec<String>;
}
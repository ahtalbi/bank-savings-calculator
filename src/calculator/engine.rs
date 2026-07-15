use crate::calculator::types::BudgetResult;
use crate::calculator::traits::{BudgetCalculator, BudgetFormatter};

/// engine that calculates the budget breakdown
pub struct BudgetEngine;

impl BudgetCalculator for BudgetEngine {
    fn calculate(&self, total: f64) -> BudgetResult {
        BudgetResult::new(total)
    }
}

/// simple text formatter for the budget
pub struct TableFormatter;

impl BudgetFormatter for TableFormatter {
    fn format_breakdown(&self, result: &BudgetResult) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("total: {:.2} mad", result.total));
        lines.push(String::new());
        lines.push(format!("{:<30} {:>10} {:>15}", "category", "%", "amount"));
        lines.push("-".repeat(57));

        for line in &result.lines {
            lines.push(format!(
                "{:<30} {:>8.0}% {:>12.2} mad",
                line.category.label(),
                line.percentage,
                line.amount
            ));
        }

        lines.push("-".repeat(57));
        lines.push(format!(
            "{:<30} {:>8.0}% {:>12.2} mad",
            "total",
            result.total_percentage(),
            result.total_amount()
        ));

        lines
    }
}
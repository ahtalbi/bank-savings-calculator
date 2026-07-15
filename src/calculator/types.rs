/// enum representing budget categories with fixed percentages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BudgetCategory {
    Rent,
    Taxes,
    Living,
    LongTermInvestment,
    ShortTermInvestment,
}

impl BudgetCategory {
    pub fn percentage(&self) -> f64 {
        match self {
            BudgetCategory::Rent => 25.0,
            BudgetCategory::Taxes => 5.0,
            BudgetCategory::Living => 40.0,
            BudgetCategory::LongTermInvestment => 20.0,
            BudgetCategory::ShortTermInvestment => 10.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            BudgetCategory::Rent => "Rent",
            BudgetCategory::Taxes => "Taxes/fees",
            BudgetCategory::Living => "Living",
            BudgetCategory::LongTermInvestment => "Long-term investment",
            BudgetCategory::ShortTermInvestment => "Short-term investment",
        }
    }
}

/// a single budget line item with category percentage and computed amount
#[derive(Debug)]
pub struct BudgetLine {
    pub category: BudgetCategory,
    pub amount: f64,
    pub percentage: f64,
}

impl BudgetLine {
    pub fn new(category: BudgetCategory, total: f64) -> Self {
        let percentage = category.percentage();
        let amount = (total * percentage / 100.0 * 100.0).round() / 100.0;
        BudgetLine { category, amount, percentage }
    }
}

/// the complete budget breakdown result
#[derive(Debug)]
pub struct BudgetResult {
    #[allow(dead_code)]
    pub total: f64,
    pub lines: Vec<BudgetLine>,
}

impl BudgetResult {
    pub fn new(total: f64) -> Self {
        let categories = vec![
            BudgetCategory::Rent,
            BudgetCategory::Taxes,
            BudgetCategory::Living,
            BudgetCategory::LongTermInvestment,
            BudgetCategory::ShortTermInvestment,
        ];

        let lines: Vec<BudgetLine> = categories
            .into_iter()
            .map(|cat| BudgetLine::new(cat, total))
            .collect();

        BudgetResult { total, lines }
    }

    pub fn total_percentage(&self) -> f64 {
        self.lines.iter().map(|l| l.percentage).sum()
    }

    pub fn total_amount(&self) -> f64 {
        self.lines.iter().map(|l| l.amount).sum()
    }
}
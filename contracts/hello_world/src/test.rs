use crate::LendingPoolContract;
use crate::Pool;
use soroban_sdk::Symbol;

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address, String, Symbol};

    // Helper function to create unique test addresses
    fn test_address(env: &Env, identifier: u8) -> Address {
        let base_address = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
        let unique_address = format!("{}{}", base_address, identifier);
        Address::from_string(&String::from_slice(env, &unique_address))
    }

    #[test]
    fn test_initialize_pool() {
        let env = Env::default();
        let pool = LendingPoolContract::initialize_pool(env.clone());

        assert_eq!(pool.total_funds, 0);
        assert_eq!(pool.contributions.len(), 0);
        assert_eq!(pool.active_loans.len(), 0);
    }

    #[test]
    fn test_contribute_to_pool() {
        let env = Env::default();
        let lender = test_address(&env, 1);
        let amount = 1000;

        LendingPoolContract::contribute_to_pool(env.clone(), lender.clone(), amount);
        let pool: Pool = env.storage().instance().get(&Symbol::new(&env, "pool")).unwrap();

        assert_eq!(pool.total_funds, amount);
        assert_eq!(pool.contributions.get(lender.clone()).unwrap().amount, amount);
    }

    #[test]
    fn test_request_loan() {
        let env = Env::default();
        let lender = test_address(&env, 1);
        let borrower = test_address(&env, 2);
        let contribution_amount = 1000;
        let principal = 500;
        let interest_rate = 10;
        let due_date = 1672531200;

        // Lender contributes to the pool
        LendingPoolContract::contribute_to_pool(env.clone(), lender.clone(), contribution_amount);

        // Borrower requests a loan
        let loan = LendingPoolContract::request_loan(env.clone(), borrower.clone(), principal, interest_rate, due_date);
        assert!(loan.is_some());

        let pool: Pool = env.storage().instance().get(&Symbol::new(&env, "pool")).unwrap();
        assert_eq!(pool.total_funds, contribution_amount - principal);
        let stored_loan = pool.active_loans.get(borrower.clone()).unwrap();
        assert_eq!(stored_loan.principal, principal);
        assert_eq!(stored_loan.interest_rate, interest_rate);
        assert_eq!(stored_loan.due_date, due_date);
        assert!(!stored_loan.repaid);
    }

    #[test]
    fn test_repay_loan() {
        let env = Env::default();
        let lender = test_address(&env, 1);
        let borrower = test_address(&env, 2);
        let contribution_amount = 1000;
        let principal = 500;
        let interest_rate = 10;
        let due_date = 1672531200;
        let repayment_amount = 550; // principal + 10% interest

        // Lender contributes and borrower requests a loan
        LendingPoolContract::contribute_to_pool(env.clone(), lender.clone(), contribution_amount);
        LendingPoolContract::request_loan(env.clone(), borrower.clone(), principal, interest_rate, due_date);

        // Borrower repays the loan
        let result = LendingPoolContract::repay_loan(env.clone(), borrower.clone(), repayment_amount);
        assert!(result);

        let pool: Pool = env.storage().instance().get(&Symbol::new(&env, "pool")).unwrap();
        assert_eq!(pool.total_funds, contribution_amount);
        let loan = pool.active_loans.get(borrower.clone()).unwrap();
        assert!(loan.repaid);
    }

    #[test]
    fn test_view_lender_balance() {
        let env = Env::default();
        let lender = test_address(&env, 1);
        let amount = 1000;

        LendingPoolContract::contribute_to_pool(env.clone(), lender.clone(), amount);
        let balance = LendingPoolContract::view_lender_balance(env.clone(), lender.clone());

        assert_eq!(balance, amount);
    }

    #[test]
    fn test_view_loan_status() {
        let env = Env::default();
        let lender = test_address(&env, 1);
        let borrower = test_address(&env, 2);
        let amount = 1000;
        let principal = 500;
        let interest_rate = 10;
        let due_date = 1672531200;

        // Lender contributes and borrower requests a loan
        LendingPoolContract::contribute_to_pool(env.clone(), lender.clone(), amount);
        LendingPoolContract::request_loan(env.clone(), borrower.clone(), principal, interest_rate, due_date);

        // View loan status (should be outstanding)
        let status = LendingPoolContract::view_loan_status(env.clone(), borrower.clone());
        assert_eq!(status, Symbol::new(&env, "Loan is outstanding."));

        // Repay loan and check status again
        let repayment_amount = 550;
        LendingPoolContract::repay_loan(env.clone(), borrower.clone(), repayment_amount);
        let status_after_repay = LendingPoolContract::view_loan_status(env.clone(), borrower.clone());
        assert_eq!(status_after_repay, Symbol::new(&env, "Loan is fully repaid."));
    }

    #[test]
    fn test_calculate_lender_earnings() {
        let env = Env::default();
        let lender1 = test_address(&env, 1);
        let lender2 = test_address(&env, 2);
        let borrower = test_address(&env, 3);
        
        let contribution_amount1 = 1000;
        let contribution_amount2 = 2000;
        let principal = 500;
        let interest_rate = 10;
        let due_date = 1672531200;
        let repayment_amount = 550;

        // Lenders contribute to the pool
        LendingPoolContract::contribute_to_pool(env.clone(), lender1.clone(), contribution_amount1);
        LendingPoolContract::contribute_to_pool(env.clone(), lender2.clone(), contribution_amount2);

        // Borrower requests and repays a loan
        LendingPoolContract::request_loan(env.clone(), borrower.clone(), principal, interest_rate, due_date);
        LendingPoolContract::repay_loan(env.clone(), borrower.clone(), repayment_amount);

        // Calculate and verify each lender's earnings
        let earnings_lender1 = LendingPoolContract::calculate_lender_earnings(env.clone(), lender1.clone());
        let earnings_lender2 = LendingPoolContract::calculate_lender_earnings(env.clone(), lender2.clone());

        // Lender1 contributed 1/3 of the pool, and Lender2 contributed 2/3, so earnings should reflect that.
        assert_eq!(earnings_lender1, repayment_amount * contribution_amount1 / (contribution_amount1 + contribution_amount2));
        assert_eq!(earnings_lender2, repayment_amount * contribution_amount2 / (contribution_amount1 + contribution_amount2));
    }
}
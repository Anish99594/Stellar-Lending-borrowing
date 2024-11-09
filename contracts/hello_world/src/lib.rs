#![cfg_attr(not(test), no_std)]
use soroban_sdk::{contractimpl, contracttype, contract, Address, Env, Symbol, Map};

#[derive(Debug, Clone)]
#[contracttype]
pub struct LoanRequest {
    borrower: Address,
    principal: i64,
    interest_rate: i64,
    due_date: u64,
    repaid: bool,
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct LenderContribution {
    lender: Address,
    amount: i64,
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Pool {
    total_funds: i64,
    contributions: Map<Address, LenderContribution>,
    active_loans: Map<Address, LoanRequest>,
}

#[contract]
pub struct LendingPoolContract;

#[contractimpl]
impl LendingPoolContract {
    // Initialize a new lending pool.
    pub fn initialize_pool(env: Env) -> Pool {
        Pool {
            total_funds: 0,
            contributions: Map::new(&env),
            active_loans: Map::new(&env),
        }
    }

    // Lender contributes to the pool.
    pub fn contribute_to_pool(env: Env, lender: Address, amount: i64) {
        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));
        pool.total_funds += amount;

        // Update lender's contribution or add a new entry.
        if let Some(mut contribution) = pool.contributions.get(lender.clone()) {
            contribution.amount += amount;
            pool.contributions.set(lender.clone(), contribution);
        } else {
            pool.contributions.set(lender.clone(), LenderContribution { lender, amount });
        }

        env.storage().instance().set(&Symbol::new(&env, "pool"), &pool);
    }

    // Borrower requests a loan.
    pub fn request_loan(env: Env, borrower: Address, principal: i64, interest_rate: i64, due_date: u64) -> Option<LoanRequest> {
        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));

        // Ensure pool has enough funds.
        if pool.total_funds < principal {
            return None;  // Insufficient funds in pool.
        }

        // Deduct the requested principal from the pool.
        pool.total_funds -= principal;

        // Create and store the loan request.
        let loan_request = LoanRequest {
            borrower: borrower.clone(),
            principal,
            interest_rate,
            due_date,
            repaid: false,
        };

        pool.active_loans.set(borrower.clone(), loan_request.clone());
        env.storage().instance().set(&Symbol::new(&env, "pool"), &pool);
        Some(loan_request)
    }

    // Borrower repays loan with potential penalty for late repayment.
    pub fn repay_loan(env: Env, borrower: Address, repayment_amount: i64) -> bool {
        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));

        if let Some(mut loan) = pool.active_loans.get(borrower.clone()) {
            if loan.repaid {
                return false;  // Loan already repaid.
            }

            // Calculate total amount due (principal + interest)
            let total_due = loan.principal + (loan.principal * loan.interest_rate / 100);

            // If repayment amount covers the loan, mark as repaid and return to pool.
            if repayment_amount >= total_due {
                loan.repaid = true;
                pool.total_funds += repayment_amount;
                pool.active_loans.set(borrower.clone(), loan);

                env.storage().instance().set(&Symbol::new(&env, "pool"), &pool);
                return true;
            } else {
                return false;  // Insufficient repayment amount.
            }
        } else {
            return false;  // No loan found for this borrower.
        }
    }

    // View contribution balance for a lender.
    pub fn view_lender_balance(env: Env, lender: Address) -> i64 {
        let pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));
        pool.contributions.get(lender.clone()).map_or(0, |contribution| contribution.amount)
    }

    // View loan status for a borrower.
    pub fn view_loan_status(env: Env, borrower: Address) -> Symbol {
        let pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));
        match pool.active_loans.get(borrower.clone()) {
            Some(loan) => {
                if loan.repaid {
                    Symbol::new(&env, "Loan is fully repaid.")
                } else {
                    Symbol::new(&env, "Loan is outstanding.")
                }
            },
            None => Symbol::new(&env, "No loan found for this borrower."),
        }
    }

    // Calculate potential earnings for a lender based on pool contributions.
    pub fn calculate_lender_earnings(env: Env, lender: Address) -> i64 {
        let pool: Pool = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "pool"))
            .unwrap_or_else(|| Self::initialize_pool(env.clone()));
        let total_interest_earned: i64 = pool.active_loans.iter()
            .map(|(_, loan)| loan.principal * loan.interest_rate / 100)
            .sum();

        // Calculate lender's share of earnings based on their contribution to the total pool.
        pool.contributions.get(lender.clone()).map_or(0, |contribution| {
            total_interest_earned * contribution.amount / pool.total_funds
        })
    }
}

mod test;
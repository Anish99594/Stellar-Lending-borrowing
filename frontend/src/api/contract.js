import { Contract, Env, Address, Symbol } from '@stellar/soroban-sdk';

const env = new Env();

const contractAddress = "contract_address"; // Replace with actual address

export const initializePool = async () => {
    const contract = new Contract(env, contractAddress);
    return await contract.invoke('initialize_pool', {});
};

export const contributeToPool = async (lenderAddress, amount) => {
    const contract = new Contract(env, contractAddress);
    await contract.invoke('contribute_to_pool', {
        lender: new Address(lenderAddress),
        amount: amount
    });
};

export const requestLoan = async (borrowerAddress, principal, interestRate, dueDate) => {
    const contract = new Contract(env, contractAddress);
    return await contract.invoke('request_loan', {
        borrower: new Address(borrowerAddress),
        principal: principal,
        interest_rate: interestRate,
        due_date: dueDate
    });
};

export const repayLoan = async (borrowerAddress, repaymentAmount) => {
    const contract = new Contract(env, contractAddress);
    return await contract.invoke('repay_loan', {
        borrower: new Address(borrowerAddress),
        repayment_amount: repaymentAmount
    });
};

export const viewLenderBalance = async (lenderAddress) => {
    const contract = new Contract(env, contractAddress);
    return await contract.invoke('view_lender_balance', {
        lender: new Address(lenderAddress)
    });
};

export const viewLoanStatus = async (borrowerAddress) => {
    const contract = new Contract(env, contractAddress);
    return await contract.invoke('view_loan_status', {
        borrower: new Address(borrowerAddress)
    });
};

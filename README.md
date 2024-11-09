# Lending Pool Platform

This project implements a decentralized lending pool system on the Soroban blockchain using Rust and the Soroban SDK. The system allows lenders to contribute to a shared pool, borrowers to request loans, and borrowers to repay loans. The platform calculates the earnings for each lender based on their contribution to the pool.

## Table of Contents
1. [Project Setup](#project-setup)
2. [Smart Contract Overview](#smart-contract-overview)
3. [Frontend Overview](#frontend-overview)
4. [API Overview](#api-overview)
5. [Running the Project](#running-the-project)
6. [Testing](#testing)
7. [Contributing](#contributing)
8. [License](#license)

---

## Project Setup

To get started with the project, you'll need to have the following installed:
- React (for the frontend application)
- Soroban SDK (for interacting with the smart contract)

Follow these steps to set up the project:

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <project-directory>
2. Install dependencies for the frontend:
     npm install

3. Replace contract_address in src/api/contract.js with the deployed contract address on the Soroban blockchain.

4. Start the frontend server:
    npm start

This will run the application on http://localhost:3000.

Smart Contract Overview
The smart contract is implemented in Rust using the Soroban SDK. It handles the core logic of the lending pool, including:

Initializing the pool
Accepting contributions from lenders
Allowing borrowers to request loans
Processing loan repayments
Viewing lender balances and loan statuses

Key Structures
  LoanRequest: Contains information about a loan, including the borrower, principal, interest rate, due date, and repayment status.
  LenderContribution: Stores a lender's contribution to the pool.
  Pool: Tracks the total funds in the pool, lender contributions, and active loans.

Key Functions
  initialize_pool: Initializes a new lending pool.
  contribute_to_pool: Allows lenders to contribute funds to the pool.
  request_loan: Allows borrowers to request a loan, provided there are enough funds in the pool.
  repay_loan: Allows borrowers to repay loans, with a check for total repayment and penalties for late repayment.
  view_lender_balance: Allows a lender to view their contribution balance.
  view_loan_status: Allows a borrower to view the status of their loan.
  calculate_lender_earnings: Calculates the potential earnings for a lender based on the pool's total interest earned.

Frontend Overview
  The frontend is a React application that allows users to interact with the lending pool contract. It consists of the following components:
    Pool: Displays the total funds in the pool and provides an option to initialize the pool.
    Contribution: Allows lenders to contribute funds to the pool.
    LoanRequest: Allows borrowers to request a loan.
    Repayment: Allows borrowers to repay their loans.

Key Files
  public/index.html: The HTML template for the React app.
  src/api/contract.js: Contains the functions that interact with the Soroban smart contract.
  src/components/Pool.js: Displays pool data and initializes the pool.
  src/components/Contribution.js: Handles lender contributions to the pool.
  src/components/LoanRequest.js: Handles loan requests from borrowers.
  src/components/Repayment.js: Handles loan repayments.
  src/App.js: Main app file with routing for different components.
  src/styles.css: Basic CSS styling for the app.

API Overview
  The API functions in src/api/contract.js interact with the Soroban smart contract. The key functions are:
    initializePool: Initializes the lending pool.
    contributeToPool: Allows lenders to contribute funds to the pool.
    requestLoan: Allows borrowers to request a loan.
    repayLoan: Allows borrowers to repay a loan.
    viewLenderBalance: Returns the balance of a lender.
    viewLoanStatus: Returns the status of a borrower's loan.

Running the Project
To run the project locally:

1. Install dependencies:
    npm install

2. Replace the contract_address in src/api/contract.js with the actual deployed contract address.

3. Run the frontend server:
    npm start
4. Navigate to http://localhost:3000 to interact with the lending pool platform.

Testing
For testing the smart contract, create unit tests in the tests folder using Soroban’s testing framework. You can run the tests with the following command:
  cargo test

For the frontend, you can use Jest or React Testing Library to test components and API functions. Install testing libraries if needed:
  npm install --save-dev jest @testing-library/react

Contributing
Feel free to contribute to the project! Here's how you can get started:
  Fork the repository.
  Create a new branch for your feature or bug fix.
  Make your changes and commit them.
  Open a pull request describing your changes.

License
  This project is licensed under the MIT License. See the LICENSE file for details.

This README provides the necessary instructions to set up, run, and contribute to the Lending Pool Platform. If you encounter any issues or need further assistance, feel free to open an issue in the repository.

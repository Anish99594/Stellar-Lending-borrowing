import React, { useState } from 'react';
import { repayLoan } from '../api/contract';

export default function Repayment() {
    const [address, setAddress] = useState('');
    const [amount, setAmount] = useState('');

    const handleRepayment = async () => {
        const success = await repayLoan(address, parseInt(amount));
        alert(success ? 'Loan repaid successfully!' : 'Repayment failed.');
    };

    return (
        <div>
            <h3>Repay Loan</h3>
            <input type="text" placeholder="Borrower Address" onChange={(e) => setAddress(e.target.value)} />
            <input type="number" placeholder="Repayment Amount" onChange={(e) => setAmount(e.target.value)} />
            <button onClick={handleRepayment}>Repay Loan</button>
        </div>
    );
}

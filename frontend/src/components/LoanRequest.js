import React, { useState } from 'react';
import { requestLoan } from '../api/contract';

export default function LoanRequest() {
    const [address, setAddress] = useState('');
    const [principal, setPrincipal] = useState('');
    const [interestRate, setInterestRate] = useState('');
    const [dueDate, setDueDate] = useState('');

    const handleRequestLoan = async () => {
        const loan = await requestLoan(address, parseInt(principal), parseInt(interestRate), parseInt(dueDate));
        if (loan) alert('Loan requested successfully!');
        else alert('Loan request failed!');
    };

    return (
        <div>
            <h3>Request a Loan</h3>
            <input type="text" placeholder="Your Address" onChange={(e) => setAddress(e.target.value)} />
            <input type="number" placeholder="Principal" onChange={(e) => setPrincipal(e.target.value)} />
            <input type="number" placeholder="Interest Rate" onChange={(e) => setInterestRate(e.target.value)} />
            <input type="date" onChange={(e) => setDueDate(new Date(e.target.value).getTime() / 1000)} />
            <button onClick={handleRequestLoan}>Request Loan</button>
        </div>
    );
}

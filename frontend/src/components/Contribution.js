import React, { useState } from 'react';
import { contributeToPool } from '../api/contract';

export default function Contribution() {
    const [address, setAddress] = useState('');
    const [amount, setAmount] = useState('');

    const handleContribute = async () => {
        await contributeToPool(address, parseInt(amount));
        alert('Contribution successful!');
    };

    return (
        <div>
            <h3>Contribute to Pool</h3>
            <input type="text" placeholder="Your Address" onChange={(e) => setAddress(e.target.value)} />
            <input type="number" placeholder="Amount" onChange={(e) => setAmount(e.target.value)} />
            <button onClick={handleContribute}>Contribute</button>
        </div>
    );
}

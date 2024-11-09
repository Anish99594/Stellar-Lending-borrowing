import React, { useEffect, useState } from 'react';
import { initializePool } from '../api/contract';

export default function Pool() {
    const [pool, setPool] = useState(null);

    useEffect(() => {
        async function fetchPool() {
            const poolData = await initializePool();
            setPool(poolData);
        }
        fetchPool();
    }, []);

    return (
        <div>
            <h2>Lending Pool</h2>
            {pool ? (
                <div>
                    <p>Total Funds: {pool.total_funds}</p>
                </div>
            ) : (
                <button onClick={initializePool}>Initialize Pool</button>
            )}
        </div>
    );
}

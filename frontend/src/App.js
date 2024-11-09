import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Pool from './components/Pool';
import Contribution from './components/Contribution';
import LoanRequest from './components/LoanRequest';
import Repayment from './components/Repayment';
import './styles.css';

function App() {
    return (
        <Router>
            <div>
                <h1>Lending Pool Platform</h1>
                <nav>
                    <a href="/">View Pool</a> | 
                    <a href="/contribute">Contribute</a> | 
                    <a href="/request-loan">Request Loan</a> | 
                    <a href="/repay-loan">Repay Loan</a>
                </nav>
                <Switch>
                    <Route path="/" exact component={Pool} />
                    <Route path="/contribute" component={Contribution} />
                    <Route path="/request-loan" component={LoanRequest} />
                    <Route path="/repay-loan" component={Repayment} />
                </Switch>
            </div>
        </Router>
    );
}

export default App;

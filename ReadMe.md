## Disclaimer
This project was made for educational and amusement purposes only.  

Any usage of the code of any sort for any reason and for any purpose is out of my responsibility.  

## What is this ?
#### Fancy talk
A stochastic investment lifecycle simulator demonstrating probabilistic wealth redistribution, background processing, and evolving account state.
#### Long story short
A basic ponzi scheme management system.

## Why is it made ?
I was watching the final episodes of season 8 of Two and a Half Men, where a character named Alan was running a Ponzi scheme, collecting money from new investors to pay existing ones. Alan was a naive, broke, divorced single father, so pathetic that I actually wanted his scam to succeed, just so he could finally have enough money to move off his brother’s couch and get a place of his own. In the show, his scam has failed, but from my perspective, he seemed to be doing well — he just needed a “management system”.

## What is the approach ?
The software randomly changes the balances of registered accounts to mimic real investment actions, but in a way that makes recent investors start winning before eventually losing over time.
> New investors win and old ones lose.
> Paying the profits won by new investors with the money lost by old ones.

When the old losses and the recent wins fluctuate, that creates a difference between the total balance of all accounts and the amount currently in the pot, known as the ***Net Pot***. 

The system then takes a small percentage from that net pot, an action known as a ***Milking Attempt***, which goes straight into your pocket as the Earned Amount.
>The Net Pot fluctuates between negative and positive values.

## How to use it ?
Open terminal and navigate to where you want the project to be located and clone the repo through:
```
1. git clone https://github.com/AhmedAyachi/Ponzimulator 
2. cd Ponzimulator
3. cargo run
```
And you get into an interactive terminal environment.
The ***cargo run*** command will run the project in a debug environment, creating a **Cache** folder to save progress inside the project root directory.

### Cashing investors in
To create a new account for an investor, type e.g.
```
> cashin ayachi 2300
owner: ayachi
account id: m3iI8YeXPAKO9q4y5bwX
```
This creates a new account with a balance of 2300 and ayachi as an owner.
The terminal will output the account id.
You can add an amount to an existing account by typing:
```
> cashin ayachi m3iI8YeXPAKO9q4y5bwX 100
✔ 100 has been deposited in ayachi's account m3iI8YeXPAKO9q4y5bwX.
```
This adds 100 to the balance of the account with the specified id and owner name if it exists.

To cash an account out, use the ***cashout*** command:
```
> cashout ayachi m3iI8YeXPAKO9q4y5bwX 300
✔ 300 has been withdrawn from ayachi's account m3iI8YeXPAKO9q4y5bwX.
```
> Not specifying an amount will cashout the whole account.

To check an account details, use the ***select*** command:
```
> select m3iI8YeXPAKO9q4y5bwX
Owner: ayachi
Deposit: 2100
Balance: 1610.9158178147734
Created on: 01/07/2026 16:39:43
Last deposit at: 01/07/2026 16:39:43
```

### Start the daemon
To start the daemon, you simply type: 
```
> start
```
The daemon is a background process that alters the balance of all accounts based on [multiple factors](#how-does-it-work-) in a way that old investors lose and new ones win.
### Stop the daemon
To stop the daemon, you simply type:
```
> stop
```

> Exiting the interactive terminal won't stop the daemon. 
> To do so, you must call stop explicitly in the interactive terminal.

### Check the daemon progress
To check the daemon progress, type:
```
> status
Daemon: running
Net Pot: 80.32195464908727
Total Pot: 49248.79604313024
Total Deposit: 49420
Earned Amount: 171.2039568697284
Accounts Count: 18
```
The ***status*** command displays information about the daemon work and gives you an idea about the state of the current investors.

|Term|Description|
|-----|-----------|
|Net Pot|A positive Net Pot means that the investors are losing. In other words, if all investors cashout at that exact moment, you got to walk away with, in this case, 80 of whatever currency you're using. A negative Net Pot means that they are winning, so basically means, you're losing and if they all cashout you need to pay that amount more.|
|Total Pot|Is the difference between the Total Deposit and the Earned Amount. Which means, it's the amount of money left after a successful milking attempt.|
|Total Deposit|Is the total amount deposited by all investors.|
|Accounts Count|How many accounts are currently registered.|

### Check current accounts
To view all currently registered accounts in a table format, type:
```
> list
```
![](https://raw.githubusercontent.com/AhmedAyachi/RepoIllustrations/refs/heads/main/Ponzimulator/ListCmdOutput.png)
> Screenshot taken on 18/07/2026.

> Type ***help*** to check all available commands.

## How does it work ?
The system checks how an account should more probably lose or win based on multiple factors including, the account creation time and the account's last deposit time.

> An account will never 100% win or lose at any point in time.

The system accomplishes that by assigning what we call a ***cota*** to each account.
A cota is an inclusive range, e.g. *[-0.5,0.7]*, that is generated based on some account details:
1. The creation time
2. The last deposit time
3. The current balance

And the account's balance fluctuates by a random number generated from that range.

Shifting the range along the x-axis controls how often the account wins or loses. 

Increasing the range width controls how much the account wins or loses.

A range with positive bounds means that the account will 100% win.

A range with negative bounds means that the account will 100% lose.

A cota will always have a negative and positive bounds to make sure that any account can lose or win at any point in time.

The system calculates what the lower and upper bounds are at a specific time to control how often and how much an account should win or lose.

For a recent account, the initial base cota starts at e.g. *[-0.1,0.12]*, and over time, it tends to shift toward e.g. *[-0.5,0.3]*.

> A base quota isn’t the account’s actual quota, since that also depends on the account’s current balance.

The starting cota makes the account more likely to win, while the ending cota makes it more likely to lose.

### Cota Shifting
As time goes forwards, a recent account cota should shift from a winning cota to a losing one.
```
lowerBound = -O.1 + coef * ( -0.5 - -O.1 );
upperBound = 0.12 + coef * ( 0.3 - 0.12 );
```

The coef value determines how much the bound is adjusting. 

The coef function should meet few conditions:  
1. Increasing.
2. Its values range between 0 and 1.  
3. Increases gradually, starting slow and then speeding up.  
# iCrosschain token swap on solana

User can call program to make swap to DEX (Raydium) or operator can call with permission for make swap or transfer USC to client account.
The program will cache the processed transaction hash to avoid duplicate transaction.

## To run test
```anchor test --skip-deploy```

## build
```anchor build```

## Deploy

```anchor deploy```

### request solana for deploy on dev

```solana config set --url https://api.devnet.solana.com```



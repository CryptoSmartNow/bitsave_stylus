# Bitsave flow Arbitrum Stylus

## Features
- Swap token
- Lock token
- Withdraw token
  - As stable coin
  - In original coin
  - With interest in bitsave coin
  - Penalty noted

## Flow
1. User saves in volatile token V.
2. User puts in penalty percentage they want to pay
3. User interest in bitsave's token B is calculated
4. User can choose to save in safe or risk mode

### Flow for safe mode
- Token V is converted to stable coin S. 
- Coin S is locked in user's account till maturity
- User wait till maturity time
- If user breaks saving
  - S is unlocked from user's account
  - Penalty percentage P% is removed from S.
  - Remaining S is converted back to V.
  - V is made available to user without interest B.

### Flow for risk mode
- Token V is locked directly for user

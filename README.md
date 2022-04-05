## erc20_lock_astar
# Incremental release method for locked funds
Write and deploy a simple smart contract on the Astar Network WASM environment, which allows a user to block (lock) his tokens in X days. After X full days a user can claim 100% of these tokens back. Also, there is a linear unlocking from the beginning. On the first day (right after lock) user can unlock 50% of the total locked amount + 5% every day. So, unlocking schedule looks like this:

| Day | Percentage |
| --- | ---------- |
| 1 day | 50% |
| 2 day | 55% |
| 3 day | 60% |
| 4 day | 65% |
| 5 day | 70% |
| 6 day | 75% |
| 7 day | 80% |
| 8 day | 85% |
| 9 day | 90% |
| 10 day |100% |
| 10 + days | 100% |

Also, if the user makes the premature claim (before the 10 + days), he will receive an amount of tokens according the unlocking schedule, remaining part will have been sent to the other Shibuya address: ZrtCzXdPDvi8ngmrb8SFxLfRbCoQWHZw2mY57RMZBnuYSNR


Example:


The user has locked 100 tokens.
Then, he claims this tokens back in the 8th day
So, he will receive 85% of his tokens (85 tokens) and remaining part (15% or 15 tokens) go to ZrtCzXdPDvi8ngmrb8SFxLfRbCoQWHZw2mY57RMZBnuYSNR

Locking period for the test: 10 full days

Network: Shibuya (Astar Network test network)

Language: Substrate Rust (ink!)



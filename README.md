# DeDNS Protocol

## Getting Started (Quick)
1. Install Phantom in your supported web browser of choice https://phantom.com/
2. Navigate to https://beta.solpg.io/
3. Import this repo from GitHub
4. Build, deploy, and test through the left UI panel

## Instructions

**[TODO]** Determine instructions for posting proof or off-chain proof confirmations for DNS provider uptime.

### Leasing Instructions
- **mintLeaseNFT**: Mint a non-fungible token representing the lease. Holding this NFT gives a pubkey write permission over the referenced domain / subdomain.
- **fundLease**: Fund a lease to keep it in good health over its lifespan.
- **withdrawFromLeaseBalance**: Withdraw from a lease balance to close it.

### Network Validator Instructions
- **payFromLeaseBalance** Pays a DNS provider from a lease balance if there is proof of up-time.

**[TODO] expireLease** - Invalidates the lease holder's write permission over a domain/subdoain.

### DNS Provider Instructions
**[TODO] registerDomain** - Creates a domain account on the blockchain with the domain name and DNS provider's public key.

# Security Policy

## Supported versions

| Version | Supported |
|---|---|
| main | Yes |

## Reporting a vulnerability

**Do not open a public issue for security vulnerabilities.**

Report vulnerabilities by emailing the maintainer directly. You will receive a response within 48 hours. If the issue is confirmed, a patch will be released as soon as possible.

Please include:
- A description of the vulnerability and its potential impact
- Steps to reproduce
- Any relevant Casper transaction hashes or contract addresses

## Scope

This repository covers the `SignalLogger` Casper smart contract and its deploy tooling. The on-chain contract is deployed to Casper Testnet only — no mainnet deployment is active.

## Key handling

Never commit secret key PEM files to this repository. The `.gitignore` excludes `*.pem` files. If you believe a key has been exposed, rotate it immediately and open a private security report.

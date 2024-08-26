# Headstash Deployment Workflow

Before sending a batch of proposals off for the community governance voting workflow, this blog post recaps where our progress is and the upcoming workflow for the 3rd iteration of our Headstash token distribution effort. Its suuupeer technical, but presisely because we can make these things we call software less intimidating, and to show directly how we expect to form the messages with the tools available! 

## Recap: The Headstash Journey
Like any journey, we have met roadbumps along the way. 

We have had network upgrades (see [v4.0](https://github.com/terpnetwork/terp-core/releases/tag/v4.0.0), [v4.1](https://github.com/terpnetwork/terp-core/releases/tag/v4.1.0)), custom [cosmwasm contracts](https://github.com/terpnetwork/headstash-patch) written to patch upgrades, and UI logic meshed together, specifically with the intention for this headstash airdrop to be an examplary use-case of Terp Netowrk, and the broader IBC tech-stack. There currently is a public headstash contract deployed, funded with TERP & THIOL, and able to be claimed be eligible recipients.

## Review Next Steps: Smart Contracts
Our proposed next steps include deploying a Secret Network CosmWasm VM compatible version of our headstash contract, and doing so fully through governance authorized means. This will shield any headstash claimers wallet from the public eye, and allow owners to give consent to who and when their headstash allocation can be revealed.

### 1. Clawback airdropped tokens
The current headstash contract has an entry point that governance can call to clawback funds into the community pool. This will make the current headstash contract no longer functional one we call this via the first governance proposal. 

```json
{
    "messages": [
        {
            "@type": "/cosmwasm.wasm.v1.MsgExecuteContract",
            "sender":"", // governance module addr
            "contract":"", // public headstash airdrop 
            "msg":{ 
                "clawback":{
                    "recipient": "" // also governance module addr (not actually used)
                }
            },
        }
    ]
}
```

### 2.Upload Cw-ICA-Headstash & Cw-ICA-Headstash-Owner
To have our actions taken as Terp Network be completely through our consensus-driven logic available, we can create an Interchain Account on Secret Network that is owned and controlled by a smart contract we instantiate on Terp Network. This way, we can fund the smart contract on Terp Network with the funds we want to use in the Secret Headstash distribution. This will be extremely helpful to allow us to authorize specific actions to be processed on-behalf of the governance module. 

We are modifying the [cw-ica-controller](https://github.com/srdtrk/cw-ica-controlle) implementation, to include the messages we will use for the headstash deployment.

### 3. Fund Cw-ICA-Headstash-Owner 
We then will propose to fund the cw-ica-headstash-owner with the clawed-back TERP & THIOL. We want to include additional communities to be eligible to claim TERP & THIOL. This has changed the distribution allocations being proposed for all communities, which can be found [here](https://github.com/terpnetwork/airdrop/pull/4).

### 4. Upload Secret Headstash Contract Via ICA Account. 
With our Cw-ICA-Headstash-Owner contract funded, we then will call the entry point to setup the headstash contract. This will upload the Secret Headstash code on Secret Network, allowing anyone to instantiate a headstash instance.

### 5. Grant Authorization to Headstash Deployment DAO. 
For our final governance proposal for this specific effort, we will propose to authorize a DAO called the Headstash Deployment DAO, containing a set of trusted 3rd party contributors decided on by governance. This DAO will be responsible for calling these methods on behalf of the Terp Network governance module:

- Instantiating SNIP25 contracts for TERP & THIOL
- Instantiating Secret Headstash Contract
- Converting TERP & THIOL into SNIP25 Versions
- Funding Secret Headstash Contract with SNIP25 & SCRT
- Adding all eligible addresses & amounts to Secret Headstash Contract
- Creating FeeGrant authorizations on behalf of Terp Network's ICA-Host account on Secret

All actions will be written into cw-orchestrator scripts, to allow for public audit of the process this DAO will take.

### Review: Next Steps: Claiming A Headstash Experience
The following is what we experct a general workflow for a user to claim their headstash airdrop:

### 1. Connect Ethereum Wallet 
This is the wallet that is an eligible owner of a web-3 & cannabis community focused project.

### 2. Connect Cosmos Compatible Wallet, Generate Throwaway Wallet
A throwaway wallet will be used by a headstash claimer, in order for there to be a way to send a message on secret network for the first time, without revealing their account or requiring complex on-ramping of funds to Secret Network. This throwaway wallet will recieve a FeeGrant from the Terp Network ICA Host for two specific actions

### 3. Register Throwaway Wallet for FeeGrant & Redeem 
There are two actions that we want to propose Terp Network To cover headstash claimers:
- 1. the tx gas for claiming the headstash
- 2. the tx gas for redeeming public TERP & THIOL

### 4. Generate Offline Signature With Ethereum Wallet
The next step will be a prompt that will have the headstash claimer generate an offline signature with the throw-away wallet as the content being signed. This way, there is a level of cryptographic certainty that the ethereum wallet owner also is in control of the throwaway wallet.

### 5. Claim Headstash With Throwaway Wallet 
All thats left to do is claim the headstash with the throwaway wallet. The headstash claimer will claim with the throwaway address, but will also provide their real wallet address, which is the actual address that the snip20 will recognize has a balance,. once a user has claimed

### Review: Next Steps: The Experience After Claiming A Headstash 
There are various paths that currently exist and would be able to be taken by newly claimed scrtTERP & scrtTHIOL, such as unwrapping, making use in various secret network ecosystems. We would love to hear ideas on how we can maximize the privacy of newly claimed headstash owners! please feel free to respond in this forum post, or reach out on discord.



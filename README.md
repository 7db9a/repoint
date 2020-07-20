# repoint

WIP. Check back soon.

## Basic commands

Look for repos owned by 7db9a.

$ repoint query [-q] --author 7db9a

Look for any repo's named repoint-demo.

$ repoint query [-q] repoint-demo

Get repoint's gitlab, github, or other repo urls.

$ repoint get --name repoint-demo --show-uri | fzf | xdg-open

Get repoint's public key address.

$ repoint get --name repoint-demo --show-addr

Lookup name by address.

$ repoint get --addr $addr --show-name

## Get started

Create a new account. You'll need to own a bitcoinsv address.

$ repoint create account NAME PUBADDR

It'll save the data to $HOME/.repoint.

## Add repos.

Creat a new repo in your repo's directory, such as `example/`.

$ repoint init

It creates this repoint.toml in the current directory.

[repository]
```
name = "example"
account = "repoint-demo"
address = "$bitcoin-adress"
```

## Publish

So far, everything has been local. To make it permanent, you push by signing with you private keys.

But first login.

$ repoint login --privkey $privkey

In the directory of example repo.

$ repoint push

All it does under the hood is sign an op-return with the private key.

## repoint.toml

To update account name to add urls to add tags, edit the repoint.toml

## Other

$ repoint like REPO-NAME

$ repoint follow REPO-NAME

$ repoint tip REPO-NAME AMOUNT

## Under the hood

repoint uses bitcoinsv to write op-return's using repoints protocol. You don't actually need repoint cli or any particular software to do it. No vendor lock.

### Protocol

The first op-return is the application ID. A user account only exists if this code is the first opreturn of the address.

There will be one account per addrress. There can be many repos per account.

It's okay to have a single account named after a single repo. In this case, querying exactly a single name will return exactly a single result.

Account name's can't be changed. Repo name's can't be be changed. However, the authorized user can point an account or repo to a new one.

The app will be beta released with at least 12 functioning opcodes. Next major upgrade will have less than 20. There can be no more than 33 op-codes, one op-code must be dropped for every one added. For more complex functionality, build a protocol or tech layer on top of this protocol.

0x72 is 'r', for repoiont, in hexedecimal.

`$ echo -n 'r' | perl -pe 's/(.)/sprintf("\\x%x", ord($1))/eg'`

Opcodes 0x72, 0x720, 0x7200, and 0x701, all else equal, produces the same tx signature.

```
Name                              Op-code     Op-code appendix                  Message

Instantiate repoint               0x7202     $app-ID                           none
Add account                       0x7203                                       $account-name
Update profile text               0x7204                                       $text
Update profile pic                0x7205                                       $uri
Add repo                          0x7206                                       $repo-name
Update repo description           0x7207     $repo-index                       $text
Update repo tags                  0x7208     $repo-index                       $text
Add repo url                      0x7209     $repo-index                       $uri
Remove repo url                   0x7210     $repo-index                       $uri
Like repo                         0x7211     $account-address $repo-index      none
Unlike repo                       0x7212    $account-address $repo-index      none
Flag repo                         0x7213    $account-address $repo-index      none
Unflag                            0x7214    $account-address $repo-index      none
Tip repo                          0x7215
Follow repo                       0x7216    $account-address $repo-index      none
Unfollow repo                     0x7217    $account-address $repo-index      none
Redirect account to account       0x7218    $account-address                  none
Redirect repo to repo             0x7219    $account-address $repo-index      none
```

#### AppID

The first op-return is the application ID. If this code is present before all other opreturns, the account doesn't exist.

The AppID will indicate the version of the protocol. The goal is to never break the developer interface. Never.

The app will be beta released with at least 12 functioning opcodes. Next major upgrade will have less than 20. There can be no more than 33 op-codes, one op-code must be dropped for every one added. For more complex functionality, build a protocol or tech layer on top of this protocol.

### Repo index driven

Repository is an index of values. Each time a repo is addded, a repo index is incremented. There can be many accounts under an account.

**Add repo**

`0x724 ${$repo-index + 1} $new-repo-name`

For the following fields, each has an op-code appendix of the the index of the repo it refers to.

`op-code repo-index [op-code-appendix] [msg]`

* Profile text

* Profile pic

* Repo description

* Repo tags

* Repo url

### Other protocols

Other protocols or technoloigiescan by layered on top of, such as ipfs or inter-blockchain, to extend repoint for the needs of users, platforms, and lawful authorities.

## Notes

https://memo.cash/protocol

https://github.com/bitcoin-sv-specs/op_return

https://github.com/unwriter/datapay

https://github.com/charleskca/bitcoin-sv-rpc

https://github.com/AustEcon/bitsv

https://github.com/AustEcon/bitsv/blob/master/bitsv/network/services/README.rst

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

Update account name               0x33d01
Update profile text               0x33d02
Update profile pic                0x33d03
Update repo name                  0x33d04
Update repo description           0x33d05
Update repo tags                  0x33d06
Add repo url                      0x33d07
Remove repo url                   0x33d08  Give the index of the item
Like repo                         0x33d09
Unlike repo                       0x33d010
Flag repo                         0x33d011
Unflag                            0x33d012
Tip repo                          0x33d013
Follow repo                       0x33d014
Unfollow repo                     0x33d015

### Other protocols

Other protocols can by layered on top of, such as for repo with ipfs or something.

## Notes

https://memo.cash/protocol

https://github.com/bitcoin-sv-specs/op_return

https://github.com/unwriter/datapay

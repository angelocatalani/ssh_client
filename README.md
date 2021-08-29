# SSH Client

[![Actions Status](https://github.com/angelocatalani/ssh_client/actions/workflows/main.yml/badge.svg)](https://github.com/angelocatalani/ssh_client/actions/workflows/main.yml)

Learning experiment about cli, argument parsing and ssh.

# Table of Contents

* [Usage](#usage)
* [Resources](#resources)

## Usage

First start the ssh test server:

```shell
docker compose up
```

The ssh test server supports 2 authentication methods:

- username and password: `test_user` and `test_password`.
- public and private key: placed inside `configuration/public_key` and `configuration/private_key`.

Then, we can run any command on that server:

- with username and password:

```shell
cargo run -- -c "ls" -c "ls" -c "ls"  -u "test_user" -a "0.0.0.0:2222" password "test_password"
```

- with public and private key:

```shell
cargo run -- -c "ls" -c "ls" -c "ls"  -u "test_user" -a "0.0.0.0:2222" private-key "configuration/private_key/id_rsa"
```

## Resources

[Rust SSH cli tool](https://saidvandeklundert.net/learn/2021-08-06-rust-ssh-cli-tool/)

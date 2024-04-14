<h1 align="center">ctf-commander</h1>
<p align="center">CTFCommander is a CLI tool wrapping different cybersecurity platforms. <p>
<p align="center">
    <img src="https://img.shields.io/badge/Built with Rust-grey?style=for-the-badge&logo=rust&color=%23B94700">
</p>

<br>

## Features

Supporting the following platforms, ctf-commander lets you access and interact with them from your terminal.

> NOTE: The developement is in a very early state, the code has barely received any refactoring and the tool isn't ready to be used.

[**Hack the Box**](https://www.hackthebox.com/)
- Interact with HTB machines (list machines and their information)
- Play HTB from the terminal (connect with VPN, start machine, etc..)
- TBD

[**Root-me**](https://www.root-me.org/)
- TBD

[**Try Hack Me**](https://tryhackme.com/)
- TBD

[**CTFd**](https://ctfd.io/)
- Download all challenges remotely
- Setup working directories according to the available challenge categories
- TBD

## Roadmap

**v0.1.0**
- [x] Create a CLI generic wrapper
- [x] Create an API generic wrapper
- [x] Handle secrets securely (tokens, credentials)

**v0.2.0**
- [ ] Add the APIs for each platform
  - [x] Hack the box (WIP)
  - [ ] Root-me
  - [ ] Try Hack Me
  - [x] CTFd (WIP)

**v0.3.0**
- [ ] Create subcommands for each platform
  - [x] Hack the box (WIP)
  - [ ] Root-me
  - [ ] Try Hack Me
  - [ ] CTFd

## Dependencies

- Any keyring back-end (to store secrets securely)
- openssl (needed for VPN)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## References

**APIs**
- [HackTheBox](https://documenter.getpostman.com/view/13129365/TVeqbmeq)
- [Root-me](https://api.www.root-me.org/)
- [TryHackMe](https://documenter.getpostman.com/view/18269560/UVCB9j5e)
- [CTFd](https://docs.ctfd.io/docs/api/redoc/)

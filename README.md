Raph's Netaccess automation!
=============

Needs rust, because its naturally the best language. Also requires geckodriver, which can be installed with rust.

Instructions to use
==
1. Install rust
2. Run `cargo install geckodriver` in any terminal
3. Make 2 environment variables: 
	1. LDAP_USERNAME: which has your username
	2. LDAP_PASSWORD: which has your password
4. run `cargo build --release` in this directory
5. Open windows task scheduler and schedule running this program as you see fit. I am running this program every time I log on my PC


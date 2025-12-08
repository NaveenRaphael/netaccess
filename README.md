Raph's Netaccess automation!
=============

Needs rust, because its naturally the best language.

Needs either firefox or edge, and therefore, geckodriver or edgedriver respectively.

WARNING: try not to use firefox that is handled by snap, because that does not work nicely with geckodriver.

Presently, only the firefox driver can be run in a headless mode

These values can be changed in the `.src/main.rs`

Instructions to use
==
1. Install rust. See instructions on `https://rust-lang.org/tools/install/`
2. Run `cargo install geckodriver` in any terminal
3. There are 2 ways to handle the credentials
	1. Using environment variables: Make 2 environment variables as follows 
		1. LDAP_USERNAME: which has your username
		2. LDAP_PASSWORD: which has your password
	2. Edit login.rs
		1. Change the string with "username" and "password" to the correct values
4. Edit the constants at the beginning of main.rs to set the correct constants
5. run `cargo build --release` in this directory
	1. The executable will be in `./target/release/netaccess`

Todo
==
- `cargo install geckodriver` automatically? -> Decided against this
- Password decryption/encryption.
	- Storing password in the exectuable is not generally good, but it is fine in my case; I do not expect to share my executable. And if someone were to steal my system, I have worse problems to worry about.
	- Ideally, the correct thing to do is to store the encrypted password, and the username is a file located at .netaccess or the like. However, due to the different ways windows and unix systems do this, I have decided to push this later. 

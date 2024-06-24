# EmerShell ( Work In Progress)
## A web SSH service and a secure FTP on the local network

Written in Rust, this program will have the following features:
- Can access the full file system of the computer hosting it.
- Has a login portal that allows for secure login
- Does not use cookies to store sessions to ensure security
- Frontend is templated and looks and works like a proper shell and almost like a proper SSH.
- adding a CLI tool called "transfer" that runs something in the server to send that file via FTP to me.
- pushes it's IP address to a private git repo every 2 hours.


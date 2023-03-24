# rust-mailing-poc

Just a POC of both SMTP and IMAP rust clients.

## Sender

We are using [Lettre](https://github.com/lettre/lettre) as a SMTP client.

## Receiver 

We want to use [async-imap](https://github.com/async-email/async-imap) as an IMAP client.

## IMAP/SMTP server

We are using [Greenmail](https://greenmail-mail-test.github.io/greenmail/).
They have a public docker image we can use to run an email server locally that supports both SMTP and IMAP protocols.

## TODO

- [ ] Use env_logger (just for fun actually)
- [ ] Implement email_receiver as a web server that exposes multiple endpoints to play around with IMAP commands
- [ ] Add a Makefile with commands to make it easier to test both sender and receiver
- [ ] Document both sender and receiver APIs (just to make this POC more friendly)

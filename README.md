# rust-mailing-poc

Just a POC of both SMTP and IMAP rust clients.

## Sender

We are using [Lettre](https://github.com/lettre/lettre) as a SMTP client.

## Receiver 

We use [async-imap](https://github.com/async-email/async-imap) as an IMAP client.

## IMAP/SMTP server

We are using [Greenmail](https://greenmail-mail-test.github.io/greenmail/).
They have a public docker image we can use to run an email server locally that supports both SMTP and IMAP protocols.

## Connection pool

I've implemented a connection pool manager for IMAP sessions using the [deadpool](https://github.com/bikeshedder/deadpool) crate.

The reason for doing this is that an IMAP session only allows to execute 1 command at a time, and 1 connection only can have 1 session, so in this case connection = session. 

If we want to be able to execute multiple IMAP commands at once, we need to use a connection pool, so we can create and reuse connections once they are available.

## Try out

Copy the `.env.example` file to a `.env` file 

```
cp .env.example .env
```

Build the images 

```
make build
```

Run the compose 

```
make up
```

Make the sender post a dummy message 

```
PORT=8000 make send_email
```

Call the receiver to fetch all the messages on the inbox

```
PORT=9095 make fetch_inbox
```

Stop the containers 

```
make down
```

or `<Ctrl-C>`

## TODO

- [X] Use env_logger (just for fun actually)
- [X] Implement email_receiver as a web server that exposes multiple endpoints to play around with IMAP commands
- [X] Add a Makefile with commands to make it easier to test both sender and receiver

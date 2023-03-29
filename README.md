# rust-mailing-poc

Just a POC of both SMTP and IMAP rust crates.

## SMTP Client

We are using [Lettre](https://github.com/lettre/lettre) crate for SMTP.

## IMAP client

We use [async-imap](https://github.com/async-email/async-imap) crate to communicate with an IMAP server.

## IMAP/SMTP server

We are using [Greenmail](https://greenmail-mail-test.github.io/greenmail/).
They have a public docker image we can use to run an email server locally that supports both SMTP and IMAP protocols.

## IMAP connection pool

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
curl -X POST http://localhost:<PORT>/send
```

Call the receiver to fetch all the messages on the inbox

```
curl -X GET http://localhost:$(PORT)/inbox
```

Set new flags to messages using their UIDs

```
curl --header "Content-Type: application/json" --data '{"uids": [1, 2], "flags": ["Draft", "MyCustomFlag"]}' http://localhost:<PORT>/inbox/flags
```

Stop the containers 

```
make down
```

or `<Ctrl-C>`

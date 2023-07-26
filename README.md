This is a discord bot made in rust that I use to manage my old laptop that I have converted into my home server.

## Why

I could not get a static IP and I don't want to pay for dynamic dns services such as [no-ip](https://www.noip.com/) as I don't know how I will use this server.


## Usage

No not run this in a docker container as this needs direct machine access to be able to run some command line scripts.
Use something like pm2 or a custom systemd service to start up the executable

The homeserver-bot.service would look something like this:

```toml
[Unit]
Description=A discord bot to manage your server

[Service]
User=<user e.g. root>
WorkingDirectory=<directory_of_script e.g. /root>
ExecStart=./homeserver-bot
Restart=always

[Install]
WantedBy=multi-user.target
``` 

## Commands

- `/ping`: to see if the server is alive.
- `/system`: prints out information like uptime, temperature etc (basically neofetch)
- `/run {command}`: runs a command in bash and shows the output
- `/ip`: prints out the public IP/IPv6 and the local IP as well.
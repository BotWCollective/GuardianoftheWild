# guardianofthewild rust edition

To run this, you need rust and cargo, which you can install from [rustup](https://rustup.rs).

GuardianoftheWild is configured through environment variables as shown below.

```sh
GOTW_CHAN="#botwcollective" # the channel to connect to, prefixed with a #
GOTW_NICK="guardianofthewild" # the username of the bot account
GOTW_PASS="oauth:abcd1234" # the twitch chat oauth token for the bot account
GOTW_LOG="info" # log level of the bot
```

## details
A command is a word that the bot responds to only when placed at the very start of a message, and a keyword is a word the bot will respond to no matter where
it is in the message. A prefix for commands is optional, but can help distinguish them from regular messages. The set of commands and the set of
keywords must not have any overlap. Commands and keywords are optionally case sensitive, which is specified on command creation. Many builtin
commands support switches, which are optional and placed after the command but before the rest of the information, as shown below.

```
!commands add -k=1 -cd=10000 gl Thanks, friend! 
```

Switches may be specified multiple times, and the last value is used.


## builtin commands (not all implemented yet!)

### !commands
When run without mod/broadcaster status or by a mod without roles, this will list all of the available commands.

**Switches:**

`-k`: show keywords instead of commands

### !commands add
**Basic usage:**
```
!commands add !hello Hi there!
```

This adds the command `!hello`, which will respond with the message "Hi there!"

**Switches:**

* `-k=NUMBER`: add a keyword rather than a command, and set the priority of the keyword. NUMBER must be positive, and default is 0
* `-cd=NUMBER`: make the command cooldown NUMBER milliseconds; default is 0
* `-p=mod|broadcaster|vip|anyone`: set who can run the command; default is `anyone`
* `-cs=true|false`: set case sensitivity; default is `true`
* `-t=js|static|sub`: set command type, see Command types below

**Substitution:**

Certain pieces of information can be substituted into the returned message by the bot, by using the syntax `${var}` in the command body. Available substitutions are

* `${count}`: Total number of times that the command has been called
* `${sender}`: The username of the person who called the command

**Command types:**

The bot will try to infer whether the command is a static command or has substitutions in it, but you *must* specify if you want the command to be javascript. You
can also override the inference by specifying static or sub.

**Aliases:**

This command is aliased to `+`.

### !commands del
**Basic usage:**
```
!commands del !hello
```

Removes the command `!hello`.

**Aliases:**

This command is aliased to `-`.

### !commands alias
**Basic usage:**
```
!commands alias !greetings !hello
```

This creates a command `!greetings` that is identical to `!hello`.
*Warning!* You cannot alias builtin commands!

**Aliases:**

This command is aliased to `ln`.

**Switches:**

* `-k`: creates the new alias as a keyword
* `-cs=true|false`: set case sensitivity; default is `true`

### !commands show
**Basic usage:**
```
!commands show !hello
```

Shows the output for the command or keyword `!hello`. If the command is a script, will send as much of the script as fits in the message. Otherwise, it will send
the raw unsubstituted output or static reply.

### !commands edit
**Basic usage:**
```
!commands edit !hello Hello there!
```

Same syntax and options as !commands add, but changes the existing command rather than creating a new one. Will cause an error if the requested command does not exist.

**Aliases:**

This command is aliased to `#`.


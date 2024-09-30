# Server Manager
This is a discord bot that allows members of a discord server to jointly
start/stop a minecraft server.

## Commands
1. `/start`: This command starts the server. It will not try to restart the
   server if it has already been started using this instance of the bot.
   - Options for this command come from the [configuration file](#configuration).
2. `/stop`: This command stops the server. It will not try to stop a server if
   it has already been stopped/has not been started using this instance of the
   bot.
   - Options for this command come from the [configuration file](#configuration).
3. `/ping`: This command checks to see if the server manager bot is running.
4. `/list`: This command lists the servers that this bot is managing. This is
   pulled from the [configuration file](#configuration).

## Getting Started
The executable can be placed anywhere on the computer that is running the
server, but make sure there is a configuration file as described in the
[Configuration section](#configuration).

## Configuration
In order to configure the server manager, you must have a valid configuration
file. Valid file types are listed
[here](https://docs.rs/config/latest/config/index.html). As long as they support
arrays. Note that only [toml files](https://toml.io/en/) have been tested. The
configuration file should have the name `server-man-config.<extension>` in the
same folder as the executable. These are the values used:

| Name      | Type    | Required | Description                                                                                                                                                                                                     |
| --------- | ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| token     | String  | Yes      | The application token                                                                                                                                                                                           |
| guild-id  | integer | Yes      | The guild id of the discord server where the bot will be active                                                                                                                                                 |
| notify-id | integer | Yes      | The user id of the person who will be pinged when the server will start/stop (probably the person who's running the server)                                                                                     |
| servers   | array   | Yes      | This array defines the servers that this bot manages. Each object in the array is a server object and should adhere to the [table below](#server-object). Each element in the array represents a single server. |

### Server Object
| Name       | Type   | Required | Description                                                                                              |
| ---------- | ------ | -------- | -------------------------------------------------------------------------------------------------------- |
| name       | String | Yes      | The name of the server that will be displayed to users on the discord server                             |
| dir        | String | Yes      | The directory where the jar file is located                                                              |
| server-jar | String | Yes      | The name of the jar file for the server                                                                  |
| max-mem    | String | Yes      | The maximum amount of memory (will be prepended with `-Xmx` as a server option)                          |
| max-mem    | String | Yes      | The minimum amount of memory that the server will use (will be prepended with `-Xms` as a server option) |
| java       | String | No       | Path to `java` executable. Will default to "java" if value is not provided                               |
| extra-opts | String | No       | Extra parameters to pass to the `java` command. Defaults to an empty string                              |

### Example
Here is an example config file in toml:
```
token = "apptokenisverylong01857135"
guild-id = 12345678910111213
notify-id = 13121110987654321

[[servers]]
name = "Above and Beyond"
dir = "Above+and+Beyond-1.3-Server"
server-jar = "forge-1.16.5-36.2.34.jar"
max-mem = "8G"
min-mem = "8G"
java = "C:/Program Files/Java/jre1.8.0_361/bin/java.exe"
extra-opts = "-Dsun.rmi.dgc.server.gcInterval=2147483646 -XX:+UnlockExperimentalVMOptions -XX:G1NewSizePercent=0 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M -XX:+UseG1GC"

[[servers]]
name = "Vanilla"
dir = "Vanilla Server"
server-jar = "server.jar"
max-mem = "6144M"
min-mem = "2048M"
```

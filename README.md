# Server Manager
This is a discord bot that allows members of a discord server to jointly
start/stop a minecraft server.

## Commands
1. `/start`: This command starts the server. It will not try to restart the
   server if it has already been started.
2. `/stop`: This command stops the server. It will not try to stop a server if
   it has already been stopped/has not been started.
3. `/ping`: This command checks to see if the server manager bot is running.

## Getting Started
Place the binary/executable in the same folder as the minecraft server. Make
sure there is a configuration file as described in the [Configuration
section](#configuration).

## Configuration {#configuration}
In order to configure the server manager, you must have a [toml
file](https://toml.io/en/) with the name `server-man-config.toml` in the same
folder as the executable. These are the required values:

| Name       | Type   | Required | Description                                                                                                                 |
| ---------- | ------ | -------- | --------------------------------------------------------------------------------------------------------------------------- |
| token      | String | Yes      | The application token                                                                                                       |
| guild-id   | number | Yes      | The guild id of the discord server where the bot will be active                                                             |
| notify-id  | number | Yes      | The user id of the person who will be pinged when the server will start/stop (probably the person who's running the server) |
| server-jar | String | Yes      | The name of the jar file for the server                                                                                     |
| max-mem    | String | Yes      | The maximum amount of memory (will be prepended with `-Xmx` as a server option)                                             |
| max-mem    | String | Yes      | The minimum amount of memory that the server will use (will be prepended with `-Xms` as a server option)                    |
| java       | String | No       | Path to `java` executable. Will default to "java" if value is not provided                                                  |
| extra-opts | String | No       | Extra parameters to pass to the `java` command. Defaults to an empty string                                                 |

### Example
Here is an example config file:
```
token = "apptokenisverylong01857135"
guild-id = 12345678910111213
notify-id = 13121110987654321
server-jar = "minecraft_server.1.19.3.jar"
max-mem = "6144M"
min-mem = "2048M"
java = "C:/Program Files/Java/jre1.8.0_361/bin/java.exe"
extra-opts = "-Dsun.rmi.dgc.server.gcInterval=2147483646 -XX:+UnlockExperimentalVMOptions -XX:G1NewSizePercent=0 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M -XX:+UseG1GC"
```

# Connector

```
  /$$$$$$                                                      /$$                        
 /$$__  $$                                                    | $$                        
| $$  \__/  /$$$$$$  /$$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$$ /$$$$$$    /$$$$$$   /$$$$$$ 
| $$       /$$__  $$| $$__  $$| $$__  $$ /$$__  $$ /$$_____/|_  $$_/   /$$__  $$ /$$__  $$
| $$      | $$  \ $$| $$  \ $$| $$  \ $$| $$$$$$$$| $$        | $$    | $$  \ $$| $$  \__/
| $$    $$| $$  | $$| $$  | $$| $$  | $$| $$_____/| $$        | $$ /$$| $$  | $$| $$      
|  $$$$$$/|  $$$$$$/| $$  | $$| $$  | $$|  $$$$$$$|  $$$$$$$  |  $$$$/|  $$$$$$/| $$      
 \______/  \______/ |__/  |__/|__/  |__/ \_______/ \_______/   \___/   \______/ |__/    
```                                                                            

An rust library for composting http logic similar to elixir plug.

## To do

* deciede if current routing is correct behavior or not
* all the functions that act on connector
  * Header function are done by exposing the HeaderMaps but this is probably wrong
* send_json
  * need serde
* other functions or helper macros for resources

## Completed

* Server starts and serves requests correctly.
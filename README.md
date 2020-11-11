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

* all the functions that act on connector
  * Header function are done by exposing the HeaderMaps but this is probably wrong
* Probably put things behind feature flags

## Completed

* Server starts and serves requests correctly.
* Can send json back
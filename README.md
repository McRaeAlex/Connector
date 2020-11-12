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

* Cookie handling
* Error handling is awful right now
* Probably put things behind feature flags
* Graceful shutdown
* Cross site scripting prevention
* Resource! macro which expands into route!

## Completed

* Server starts and serves requests correctly.
* Can send json back

## Demo application

Gumshoe is a demo application for the http server
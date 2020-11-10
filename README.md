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

* path matching returning a iterator of the results
* route! macro for pattern matching urls
    * need to handle converting From<String>. Currently only support string arguments.
    * When TryFrom<String> is accepted as types we will be able to match on types which is super cool.
* all the functions that act on connector
* send_json
* other functions or helper macros for resources

## Completed

* Server starts and serves requests correctly.
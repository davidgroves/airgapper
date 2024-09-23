# AirGapper.

## Purpose.

AirGapper is a simple command-line tool to generate QR codes of 8bit binary content.

This can be text, binary data or any other 8bit content.

It is designed to get data from a system which is airgapped from the network, or has a display capable of displaying QR codes but no network connection.

The original purpose was to assist with getting debug data from an embedded device on an aircraft entertainment system (ACES) to a laptop for analysis.

## Building.

`$ cargo build --release` will build a release binary in the `target/release` directory for your host architecture.

The [docker buildx](https://docs.docker.com/reference/cli/docker/buildx/) system allows you to build the binary in a container, and then copies the binary to the [bin](bin) directory.
It is preset to build two binaries, one for x86_64 but statically linked with (MUSL)[https://www.musl-libc.org/].
And another that is also crosscompiled for [armv7-unknown-linux-musleabihf[(https://en.wikipedia.org/wiki/ARM7)].

```
$ docker buildx build --target=artifact --output type=local,dest="." .
$ file bin/airgapper-x86_64-static 
bin/airgapper-x86_64-static: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, BuildID[sha1]=fd566a118358309f33ae76ebdac2c4337e460aa2, not stripped
$ file bin/airgapper-armv7-static 
bin/airgapper-armv7-static: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped
```

## Usage.

```
USAGE:
    airgapper [OPTIONS]

OPTIONS:
    -e, --ec <ec>              Error correction level to use [default: medium] [possible values: low, medium, quartile, high]
    -h, --help                 Print help information
    -i, --input <input>        Optional input file
```

Use `-e` to set the [error correction level](https://www.qrcode.com/en/about/error_correction.html).
Use `-i` to read input from a specified file instead of stdin.

## Example.

```
$ echo "Hello, world!" | airgapper
                                                        
                                                          
        ██████████████  ██  ██      ██████████████        
        ██          ██    ████  ██  ██          ██        
        ██  ██████  ██    ██  ██    ██  ██████  ██        
        ██  ██████  ██  ██████████  ██  ██████  ██        
        ██  ██████  ██  ██  ██████  ██  ██████  ██        
        ██          ██  ██  ██  ██  ██          ██        
        ██████████████  ██  ██  ██  ██████████████        
                        ██████                            
        ██      ██  ████████    ████████████    ██        
          ████████    ██  ████  ██      ████████          
        ██████      ██████  ██  ██████  ██    ██          
        ████    ██    ██      ██████                      
            ████    ██    ██  ██      ██      ██          
                        ██      ████████  ██  ████        
        ██████████████  ████████    ██  ████  ██          
        ██          ██    ██  ██████  ████                
        ██  ██████  ██  ██  ██    ██                      
        ██  ██████  ██    ████  ██      ████  ████        
        ██  ██████  ██    ██  ██      ██████              
        ██          ██      ████  ██                      
        ██████████████  ██    ██████  ████      ██        
```
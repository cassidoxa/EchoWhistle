# Overview

Echo Whistle is a proof-of-concept network item service for [Free Enterprise](http://ff4fe.com).
A client, server, and patch are provided. The user can edit an included YAML file to change
which items are received from a small subset of item locations.

# Requirements

Echo Whistle requires SNI (The latest release as of publishing can be found [here](https://github.com/alttpo/sni/releases/tag/v0.0.90). It will
(probably) not work with QUsb2Snes.

The following platforms should work:
* FXPak
* [snes9x-emunwa](https://github.com/Skarsnik/snes9x-emunwa/releases/tag/1.61-nwa-beta2)
* [bsnes-plus-wasm](https://github.com/alttpo/bsnes-plus-wasm/releases/tag/nightly)
* Retroarch with a bsnes-mercury core and Network Commands set to On.

# Instructions
Check the releases tab for a pre-built package for your OS.

## Building the ROM

Two bps patches are included to be applied to a vanilla ROM. One will produce the original Free Enterprise
ROM, the one named "echowhistle.bps" will produce the ROM that will work with the item service.
You can use an in-browser patcher such as this one: https://www.marcrobledo.com/RomPatcher.js/

## Running the Server and Client

Executables are included in the packaged release or can be built from source with Cargo. The server
will expect a file named `secrets_ki.yaml` in the same directory it's being executed from or an
optional `--yaml path/to/yaml` parameter can be passed via CLI. Both the server and client can take
optional `--host` and `--port` parameters, by default 127.0.0.1:38281.

## Playing the Game

You can edit the yaml file describing a small subset of key item-location pairs before running the
server. Instructions are included in the document. The suggested order of operations is as follows:

1. Start SNI
2. Load the ROM and wait on the title screen
3. Start the client & server
4. Begin playing the game

# Issues and Advisories

* __No characters can join my party including one of the starting characters__

Sorry. Characters are available in the Mysidia crystal room.

* __The whistle doesn't work__

Sorry.

* __One of the item locations contains the gauntlet fight__

Very sorry.

* __This only works on the provided patch__

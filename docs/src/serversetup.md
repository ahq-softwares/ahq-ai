---
title: Server Setup
editLink: true
---

<script>
</script>
# Server Setup

## Prerequisites
The following softwares you will need to install are:
1. Ollama
2. Rust Programming Tools

To install ollama you can go to [this](https://ollama.com/download) link and download for your appropriate operating system.

To install Rust and Microsoft Visual Build Tools, you can follow these links:

[Rust](https://rust-lang.org/tools/install/) and
[Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

## Installing Dependencies
Now that you have setup Rust and VS Build Tools, you can now start installing the required dependencies. Using the following command:

```bash
cargo build
```

## Installing Ollama Models

To install a LLM model for your server to run you have to look through the ollama model registry [here](https://ollama.com/search). Once you find the required model you can install it with the following command:
```bash
ollama pull (modelID)
```
This will install the model of which you would like to run.

### Recommended Models
Some recommended models for you to run if you are running this on a normal pc is:

**llama3:7b** for computers with a good GPU and RAM.

**gemma3:270m** for computers with a mid to weak GPU and low RAM.

## Running the configuration wizard
To setup the server with the local configuration for your server to ensure the server and ollama can talk to each other follow these steps:

Step 1: To run the configuration wizard run the following command:
```bash
cargo run -- config
```
You will get a output like the picture below, here you can update the configuration of the server.
![Step 1](https://cdn.discordapp.com/attachments/1431916130393526282/1433154381313871952/image.png?ex=6903a887&is=69025707&hm=6ad9d650d1963a3d6f023a2466a95e82c649a153d46093d32916516ef80ac523)

Step 2: To configure ollama use your arrow keys to navigate to the ollama section.
![Step 2.1](https://cdn.discordapp.com/attachments/1431916130393526282/1433153531715653702/image.png?ex=6903a7bc&is=6902563c&hm=9ccffb9ce1dd53c7054e20ca8012557cabbb7e3e16735fa73425934ed7d39ffc&)

Then, select the hostname and the ports of the ollama server. You can find these out by running 
```bash
ollama serve
```
and updating the port by clicking enter on your keyboard.
![Step 2.1](https://cdn.discordapp.com/attachments/1431916130393526282/1433153531967439050/image.png?ex=6903a7bc&is=6902563c&hm=ae476bae2f09891a0497cb9cced8bafaee1b4353adc773c8cc6c902e50e63005&)
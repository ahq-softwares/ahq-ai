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
![Step 1](/images/image1-1.png)

Step 2: To configure ollama use your arrow keys to navigate to the ollama section.
![Step 2.1](/images/image1.png)

Then, select the hostname and the ports of the ollama server. You can find these out by running 
```bash
ollama serve
```
and updating the port by clicking enter on your keyboard.
![Step 2.1](/images/image2.png)
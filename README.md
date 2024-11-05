# Solana Node Management and Task Dispatch Program

This project provides a Solana program and client application to manage nodes and dispatch tasks across nodes, built using Solana's Rust SDK. The program defines four main instructions:
1. **RegisterNode**: Registers a new node with a specified ID.
2. **RemoveNode**: Removes a node based on its ID.
3. **DispatchTask**: Dispatches a task to a node, specifying a unique task ID.
4. **ReturnAnswer**: Returns a response or answer from a node for a given task.

The project also includes a TypeScript client for interacting with the Solana program, making it easy to manage nodes and execute tasks directly from a client environment.

---

## Prerequisites

- **Node.js** and **npm**: Required to run the TypeScript client.
- **Rust and Cargo**: Required to build and deploy the Solana program.

**Note**: For easy testing and deployment, you can use the [Solana Playground](https://beta.solpg.io), an in-browser development environment for Solana, eliminating the need for a full local Solana setup.

---

## Quick Start Guide

### Setting Up Solana Playground

Solana Playground provides a quick, convenient way to develop on Solana without a local setup.

1. Open [Solana Playground](https://beta.solpg.io).
2. In the bottom left corner, create or connect a wallet, which will be used for deploying and testing the program.
3. Copy or write the Rust code for your program in the Playground.
4. Use the **Build** button to compile the program.
5. Use the **Deploy** button to deploy the program to the Playground network, which assigns a `PROGRAM_ID`.
6. Copy the `PROGRAM_ID`, as you’ll need it in the client code to interact with the program.


### Available Instructions
The client provides interactions for all four instructions in the Solana program:

RegisterNode: Registers a node by providing its ID.
RemoveNode: Removes a node by ID.
DispatchTask: Sends a task to a specified node.
ReturnAnswer: Retrieves a response from a node for a dispatched task.
These instructions are serialized and sent as transactions to the program, with nodeId and taskId as parameters.


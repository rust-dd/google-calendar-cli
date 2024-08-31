# Google Calendar CLI - gcal

![example workflow](https://github.com/rust-dd/google-calendar-cli/actions/workflows/rust.yml/badge.svg)

The Google Calendar CLI (gcal) is a command-line interface (CLI) tool written in Rust, designed to make managing your Google Calendar from the terminal both quick and easy. Whether you're adding new events or listing existing ones, gcal allows you to do it all without leaving your terminal.


*Note: This project is currently in its early stages. While it is functional, many features are still under development. Consider it a playground for exploration and experimentation.*

![Screenshot](docs/screenshot.png)

Happy scheduling!

***

## Installation

To get started with gcal, clone the repository and build the project using Cargo:


```sh
git clone git@github.com:zeldan/google-calendar-cli.git
cd google-calendar-cli
cargo build && cargo install --path . --locked
```

## Usage


### Help Command

To view available commands and options, use:


```sh
gcal help
```

### Example Commands

Here are some example commands to help you get started:


| Description                          | Command                                          |
|--------------------------------------|--------------------------------------------------|
| Quick event for today                | `gcal "Retro & Demo at 16:00"`                   |
| Quick event on a specific date       | `gcal "Appointment on June 3rd 10am-10:25am"`    |
| Add event specifying only the time   | `gcal "Appointment" "10:25"`                     |
| Add event with month and day         | `gcal "Appointment" "07-13 23:25"`               |
| Add event with full date and time    | `gcal add "Appointment" "2024-07-12 10:25"`      |
| Add event with conference meeting    | `gcal "Appointment" "23:45" --conference`        |
| List events                          | `gcal list`                                      |


## Authentication

To use gcal, you'll need to authenticate with your Google account. The project includes a default, hardcoded Google API secret, which is suitable for temporary use but has a user cap. For long-term usage, or if you hit the user cap, you can set up custom authentication via Google Console.

### Setting Up Custom Authentication

If you prefer to use your own Google API credentials, follow the step-by-step instructions provided [here](docs/custom_auth.md).

### Authentication Process

1. Run any gcal command; the authentication process will start automatically.
2. Follow the on-screen instructions to complete the authentication.
3. The authentication token will be saved to ~/.gcal/store.json for future use.


## Development

For developers looking to contribute or experiment with gcal, you can run the project directly from the source:


```sh
cargo run -- list
```

This command will compile and run the gcal tool, allowing you to list events or perform other tasks directly from your development environment.

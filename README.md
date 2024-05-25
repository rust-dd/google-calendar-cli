# Google Calendar CLI - gcal

This project is a command-line interface (CLI) tool written in Rust for managing your Google Calendar. The primary purpose is to allow users to interact with their Google Calendar directly from the terminal. Adding a new event or listing them should be very easy and quick.

*Note: This project is in a very early phase, and many features are still missing. It is currently a playground project, meant for exploration and experimentation.*



Happy scheduling!

***

## Installation

1. Clone this repository

```
git@github.com:zeldan/google-calendar-cli.git
```

2. Build the project

```
cargo build --release
```

3. Run the CLI tool
```
./target/release/gcal --help
```


## Usage


- Help

```
gcal help
```

- Quick Add Event

https://developers.google.com/calendar/api/v3/reference/events/quickAdd


```
gcal "Retro & Demo at 16:00"
```

```
gcal "Appointment on June 3rd 10am-10:25am"
```

- List events


- Delete Event



## Create Application Secret

TODO: 

1. Enable Google Calendar

https://console.cloud.google.com/apis/library/calendar-json.googleapis.com

2. Create oauth2 credentials


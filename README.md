# Google Calendar CLI - gcal

![example workflow](https://github.com/zeldan/google-calendar-cli/actions/workflows/rust.yml/badge.svg)

This project is a command-line interface (CLI) tool written in Rust for managing your Google Calendar. The primary purpose is to allow users to interact with their Google Calendar directly from the terminal. Adding a new event or listing them should be very easy and quick.

*Note: This project is in a very early phase, and many features are still missing. It is currently a playground project, meant for exploration and experimentation.*

![Screenshot](docs/screenshot.png)

Happy scheduling!

***

## Installation


```
git clone git@github.com:zeldan/google-calendar-cli.git
cd google-calendar-cli
cargo build && cargo install --path . --locked
```

## Usage


### Help

```
gcal help
```

### Add event


Quick event for today

```
gcal "Retro & Demo at 16:00"
```

Quick event on a specific date

```
gcal "Appointment on June 3rd 10am-10:25am"
```

Add event with specifying the date.


```
gcal "Appointment" "10:25"
```
```
gcal "Appointment" "07-13 23:25"
```
```
gcal add "Appointment" "2024-07-12 10:25"
```

Add event with conference meeting
```
gcal "Appointment" "23:45" --conference
```

List events

```
gcal list
```

## Google Calendar API Authentication with OAuth2

This guide will help you set up OAuth2 authentication for your Google Calendar API. Follow these steps to create a project, enable the API, and obtain the necessary credentials.

### Step 1: Create a New Project
1. Go to the [Google Developer Console](https://console.developers.google.com/).
2. Click on the **Create Project** button.
3. Enter a name for your project and click **Create**.

### Step 2: Enable the Google Calendar API
1. In the [Google Developer Console](https://console.developers.google.com/), navigate to the **Library** section.
2. Search for "Google Calendar API".
3. Click on the **Google Calendar API** and then click **Enable**.

### Step 3: Create OAuth2 Consent Screen
1. In the [Google Developer Console](https://console.developers.google.com/), navigate to the **OAuth consent screen** section.
2. Choose **External** as the user type and click **Create**.
3. Fill out the required app information:
   - **App name**: `gcalcli`
   - **User support email**: `your@email.com`
4. Fill out the required developer contact information:
   - **Email addresses**: `your@email.com`
5. Click **Save and continue**.
6. Under **Scopes**, click **Save and continue**.
7. Under **Test users**, add your email (`your@gmail.com`).
8. Click **Save and continue**.

### Step 4: Create OAuth Client ID
1. In the [Google Developer Console](https://console.developers.google.com/), navigate to the **Credentials** section.
2. Click **Create credentials** and select **OAuth client ID**.
3. Select **Application type: Desktop app**.
4. Click **Create**.
5. Download the JSON file containing your client ID and secret.

### Step 5: Configure gcalcli
1. Create a directory for gcalcli configuration:
    ```sh
    mkdir -p ~/.gcal
    ```
2. Place the downloaded JSON file into the `~/.gcal` directory:
    ```sh
    mv /path/to/your/downloaded/secret.json ~/.gcal/secret.json
    ```

### Step 6: Authenticate with Google
1. Start gcalcli. The authentication process will begin automatically.
2. Follow the instructions to complete the authentication process.

The resulting token will be stored in the `~/.gcal/store.json` file.

By following these steps, you will have successfully set up OAuth2 authentication for your Google Calendar API using gcalcli.


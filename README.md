
# gss-search

`gss-search` is a command-line application written in Rust that allows users to fuzzy find and select records from a Google Spreadsheet. The tool integrates with the Google Sheets API to fetch data, stores it locally for quick access, and provides a user-friendly CLI for searching through the data.

## Overview

- Fetch data from Google Sheets using the Google Sheets API.
- Store fetched data locally for quick access.
- Fuzzy find and select records from the local data.

## Features

- **Google Sheets API Integration**: Authenticate and fetch data from Google Sheets.
- **Local Storage and Caching**: Store fetched data in a local file-based cache.
- **Fuzzy Finder**: Search through the data using a fuzzy search algorithm.
- **User-Friendly CLI**: Simple and intuitive command-line interface.

## Installation

### Using Cargo Install

1. **Install directly from GitHub**:
   ```sh
   cargo install --git https://github.com/bonyuta0204/gss-search
   ```

### From GitHub Repository

1. **Clone the repository**:
   ```sh
   git clone https://github.com/bonyuta0204/gss-search.git
   cd gss-search
   ```

2. **Build the project**:
   ```sh
   cargo build --release
   ```

3. **Run the project**:
   ```sh
   ./target/release/gss-search --url YOUR_SPREADSHEET_URL
   ```

## Configuration

To use `gss-search`, you need to create and configure a Google Spreadsheet API OAuth2 client. Follow these steps:

1. **Create a Google Cloud Project**:
   - Go to the [Google Cloud Console](https://console.cloud.google.com/).
   - Create a new project or select an existing project.

2. **Enable Google Sheets API**:
   - Go to the [API Library](https://console.cloud.google.com/apis/library) and enable the Google Sheets API for your project.

3. **Create OAuth 2.0 Client IDs**:
   - Go to the [Credentials](https://console.cloud.google.com/apis/credentials) page.
   - Click on "Create Credentials" and select "OAuth 2.0 Client IDs".
   - Configure the consent screen if prompted.
   - Select "Desktop app" for the application type.
   - Download the `client_secret.json` file and save it in the project directory.

4. **Set up environment variables (optional)**:
   - Alternatively, you can set the following environment variables:
     - `GOOGLE_CLIENT_ID`
     - `GOOGLE_CLIENT_SECRET`
     - `GOOGLE_PROJECT_ID`

## Usage

`gss-search` allows users to fetch, cache, and fuzzy search records from Google Spreadsheets using a user-friendly command-line interface.

```sh
Usage: gss-search --url <URL>

Options:
  -u, --url <URL>
          URL for spreadsheet

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Development

### Prerequisites

- Rust: Install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Setting up the development environment

1. **Clone the repository**:
   ```sh
   git clone https://github.com/bonyuta0204/gss-search.git
   cd gss-search
   ```

2. **Build the project**:
   ```sh
   cargo build
   ```

3. **Run tests**:
   ```sh
   cargo test
   ```

4. **Run the project**:
   ```sh
   cargo run -- --url YOUR_SPREADSHEET_URL
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License.

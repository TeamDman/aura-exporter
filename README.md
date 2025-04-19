# Aura-Exporter

This is a data-preservation utility to create a backup of Aura frames.

I've had family mistakenly wipe their frame with no backup restore possible, so this is my response to that.

## Features

*   **Backup:** Downloads all assets associated with your Aura frames.
*   **Authentication:** Uses environment variables for secure login.
*   **Local Storage:** Organizes downloaded assets into a local directory structure.
*   **Resumable:**  The backup process can be stopped and resumed, downloading only missing files.
*   **Asset Selection:** Allows interactive selection of frames and assets to download.

## Getting Started

### Prerequisites

*   Rust toolchain: Install from <https://www.rust-lang.org/tools/install>
*   Environment Variables:  Set `AURA_EMAIL` and `AURA_PASSWORD`

### Installation

1.  Clone the repository:

    ```bash
    git clone <repository_url>
    cd aura-exporter
    ```

### Usage

1.  **Login:** (Required before running any other commands)

    ```pwsh
    cargo run -- login
    ```

    This command will authenticate with the Aura API using the `AURA_EMAIL` and `AURA_PASSWORD` environment variables. It will save the authentication token to `aura-auth.json`.
    **WARNING:** This file contains your credentials. Remove it when you are done by running the `logout` command.

2.  **Logout:**

    ```pwsh
    cargo run -- logout
    ```

    Removes the authentication file `aura-auth.json`.

3.  **List Frames:**

    ```pwsh
    cargo run -- frame list
    ```

    Lists all frames associated with your account, displaying their ID and name.

4.  **List Frame Assets:**

    ```pwsh
    cargo run -- frame asset list --frame-id <frame_id>
    ```

    Lists all assets associated with a specific frame, summarized by user.  Replace `<frame_id>` with the ID of the frame you want to view assets for.

5.  **Download Specific Asset:**

    ```pwsh
    cargo run -- asset download --user-id <user_id> --file-name <file_name> --save-dir <save_dir>
    ```

    Downloads a specific asset.  Requires the `user_id`, `file_name`, and `save_dir` arguments.

6.  **Interactive Asset Download:**

    ```pwsh
    cargo run -- frame asset download-picker --save-dir <save_dir>
    ```

    Allows you to interactively select frames and assets to download using a fuzzy finder.

7.  **Backup All Assets:**

    ```pwsh
    cargo run -- backup sync --save-dir <save_dir> --delay-ms 3000
    ```

    Downloads all assets associated with all frames.

    *   `<save_dir>`: The directory where you want to save the downloaded assets.
    *   `--delay-ms`:  The delay in milliseconds between asset downloads. This helps prevent rate limiting.

### Environment Variables

*   `AURA_EMAIL`: Your Aura account email address.
*   `AURA_PASSWORD`: Your Aura account password.

### Directory Structure

The downloaded assets are organized in the following directory structure:

```
<save_dir>/
    users/
        <user_id>/
            <file_name>
            ...
        ...
    ...
```

### License

[MIT](LICENSE)

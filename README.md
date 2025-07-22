
# Ilmiya Setup Instructions

This guide walks you through setting up and running the Coona service locally.

---

## 📦 Prerequisites

### 🦀 Install Rust (version 1.82.0) Using `rustup`

Install `rustup` (the recommended way to manage Rust versions):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

After installation completes, either restart your terminal or run:

```bash
source $HOME/.cargo/env
```

Then install the specific version:

```bash
rustup install 1.82.0
rustup default 1.82.0
```

Verify the installation:

```bash
rustc --version
cargo --version
```

You should see:

```
rustc 1.82.0 (f6e511eec 2024-10-15)
cargo 1.82.0 (8f40fc59f 2024-08-21)
```

---

## ⚙️ Setup Steps

1. **Copy the `.env` File**
   Copy the `.env` file into the project root.
   ⚠️ **Important:** Check and adjust the port number if you're deploying locally.

2. **Copy the `prompt.json` File**
   Place `prompt.json` into the appropriate directory as expected by the application.

3. **Start the Local Database**
   Run the following script to start the local database service:

   ```bash
   ./run-local_db.sh
   ```

   ⚠️ **Note:** Ensure the database port matches your local configuration.

4. **Populate Redis with Word-to-Word Quran Data**
   Run the corresponding Python script to add Quran data to Redis:

   ```bash
   python path/to/your_script.py
   ```

   Replace `path/to/your_script.py` with the actual script path.

5. **Build and Run Ilmiya**
   Compile the Rust project and activate the Ilmiya service:

   ```bash
   cargo build
   cargo run
   ```

---

## ✅ You're Done!

Ilmiya should now be running locally. If you encounter any issues, double-check the port settings in your `.env` and database configurations.



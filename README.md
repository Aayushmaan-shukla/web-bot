# Web AI Bot 

Web AI Bot is a powerful, context-aware Chrome Extension built with **Vite, TypeScript, and Rust WebAssembly**. It acts as your personal AI assistant, seamlessly reading the content of any webpage you are currently on and allowing you to ask questions about it directly from a sleek browser side panel.

By leveraging a highly optimized Rust Wasm backend, the extension securely routes requests to top-tier LLM providers (including OpenAI, Anthropic, Gemini, and Grok) without relying on intermediate external servers.

---

##  Features

- **Context-Aware Chat:** Automatically extracts and understands text from your currently active tab.
- **Multiple LLM Providers:** Bring your own API keys for:
  - OpenAI (GPT-4o, etc.)
  - Anthropic (Claude 3 Opus, etc.)
  - Google Gemini (1.5 Pro, etc.)
  - Grok (x.ai)
  - GLM (Zhipu AI)
- **Custom Model Selection:** Easily swap out models via the settings UI to stay up-to-date with the latest AI releases.
- **High Performance Backend:** Written in Rust and compiled to WebAssembly (Wasm) for maximum speed and secure execution within the browser's sandbox.
- **Rich Markdown Rendering:** Code blocks, bold text, and lists are natively rendered into clean HTML right in the chat interface.

---

##  Architecture

The architecture relies on the Chrome Manifest V3 standard paired with a custom WebAssembly logic core:

1. **Side Panel UI (`src/sidepanel/`)**: A Vite/TypeScript frontend that handles user interaction, renders markdown using `marked`, and saves your API credentials securely to `chrome.storage.local`.
2. **Content Script (`src/content/`)**: Injected into all webpages to securely scrape context (text) upon request from the Side Panel.
3. **Core Logic / Wasm (`core_logic/`)**: A Rust crate utilizing `wasm-bindgen`, `web-sys`, and `serde`. It formats payloads dynamically, handles HTTP requests to the LLM APIs via browser fetch, catches errors natively, and returns parsed data back to the UI.

---

##  Getting Started

### Prerequisites

To build this project from source, you will need:
- **Node.js** (v18+)
- **Yarn** (Use `corepack enable` if necessary)
- **Rust Toolchain** (Ensure the `msvc` build tools are configured if you are on Windows)
- **wasm-pack** (for compiling Rust to WebAssembly)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/web-aibot.git
   cd web-aibot
   ```

2. **Install JavaScript dependencies:**
   ```bash
   yarn install
   ```

3. **Compile the Rust Backend to WebAssembly:**
   ```bash
   cd core_logic
   wasm-pack build --target web --out-dir ../src/pkg
   cd ..
   ```

4. **Start the Development Server:**
   ```bash
   yarn dev
   ```

### Loading into Chrome

1. Open Google Chrome and navigate to `chrome://extensions/`.
2. Enable **Developer mode** in the top right corner.
3. Click on **Load unpacked** and select the newly generated `dist` folder located in your project directory.
4. Pin the extension to your toolbar, click the icon, and the side panel will open!

---

##  Usage

1. Open the Side Panel on any webpage.
2. Click the **Settings Gear** in the top right.
3. Select your preferred AI Provider.
4. (Optional) Provide a specific model name, or leave it blank to use the default.
5. Enter your API Key and click **Save & Back**.
6. Start asking questions! The bot will instantly read the page you are on and answer based on its content.

---

##  License

This project is open-source and available under the MIT License.

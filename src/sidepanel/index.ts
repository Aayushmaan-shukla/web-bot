import init, { process_llm_request } from '../pkg/core_logic.js';
import { marked } from 'marked';

// State
let apiKey = '';
let provider = 'openai';
let model = 'gpt-4o';
let isSettings = false;
let isLoading = false;
let isWasmLoaded = false;

init().then(() => {
  console.log("Rust Wasm backend logic loaded.");
  isWasmLoaded = true;
});

// DOM Elements
const chatView = document.getElementById('chatView')!;
const settingsView = document.getElementById('settingsView')!;
const settingsBtn = document.getElementById('settingsBtn')!;
const saveBtn = document.getElementById('saveBtn')!;
const sendBtn = document.getElementById('sendBtn')! as HTMLButtonElement;
const chatInput = document.getElementById('chatInput')! as HTMLInputElement;
const providerSelect = document.getElementById('providerSelect')! as HTMLSelectElement;
const modelInput = document.getElementById('modelInput')! as HTMLInputElement;
const apiKeyInput = document.getElementById('apiKeyInput')! as HTMLInputElement;
const chatHistory = document.getElementById('chatHistory')!;
const loadingIndicator = document.getElementById('loadingIndicator')!;

// Load settings from storage
chrome.storage.local.get(['apiKey', 'provider', 'model'], (result) => {
  if (result.apiKey) {
    apiKey = result.apiKey;
    apiKeyInput.value = apiKey;
  }
  if (result.provider) {
    provider = result.provider;
    providerSelect.value = provider;
  }
  if (result.model) {
    model = result.model;
    modelInput.value = model;
  }
});

// Navigation
settingsBtn.addEventListener('click', () => {
  isSettings = true;
  chatView.classList.add('hidden');
  settingsView.classList.remove('hidden');
});

saveBtn.addEventListener('click', () => {
  apiKey = apiKeyInput.value.trim();
  provider = providerSelect.value;
  model = modelInput.value.trim();
  if (!model) {
    // defaults based on provider
    const defaults: Record<string, string> = {
        'openai': 'gpt-4o',
        'anthropic': 'claude-3-opus-20240229',
        'gemini': 'gemini-1.5-pro',
        'grok': 'grok-beta',
        'glm': 'glm-4'
    };
    model = defaults[provider] || 'gpt-4o';
    modelInput.value = model;
  }

  chrome.storage.local.set({ apiKey, provider, model });
  
  isSettings = false;
  settingsView.classList.add('hidden');
  chatView.classList.remove('hidden');
});

// Chat Logic
function addMessage(content: string, role: 'user' | 'system') {
  const div = document.createElement('div');
  div.className = `message ${role}`;
  if (role === 'system') {
    div.innerHTML = marked.parse(content) as string;
  } else {
    div.textContent = content;
  }
  chatHistory.appendChild(div);
  chatHistory.scrollTop = chatHistory.scrollHeight;
}

function setLoading(loading: boolean) {
  isLoading = loading;
  sendBtn.disabled = loading;
  if (loading) {
    loadingIndicator.classList.remove('hidden');
  } else {
    loadingIndicator.classList.add('hidden');
  }
}

async function handleSend() {
  const text = chatInput.value.trim();
  if (!text || isLoading) return;

  if (!apiKey) {
    addMessage("Please enter an API Key in the settings.", "system");
    return;
  }

  if (!isWasmLoaded) {
    addMessage("Initializing backend logic, please wait...", "system");
    return;
  }

  // Add user message to UI
  addMessage(text, 'user');
  chatInput.value = '';
  setLoading(true);

  try {
    // 1. Get active tab context
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    let context = "";
    if (tab && tab.id) {
      try {
        const response = await chrome.tabs.sendMessage(tab.id, { type: "GET_CONTEXT" });
        if (response && response.payload) {
          context = response.payload;
        }
      } catch (e) {
        console.warn("Content script not found on this page.");
        addMessage("⚠️ Could not read the page content. Please refresh the web page you are trying to read and try again!", "system");
        setLoading(false);
        return; // Don't proceed with the LLM call if we can't get context
      }
    }

    // 2. Call Rust Backend Logic
    console.log(`[JS] Initiating Rust Wasm call... Provider: ${provider}, Model: ${model}`);
    console.time('[JS] Rust Wasm total execution');
    const answer = await process_llm_request(provider, apiKey, model, text, context);
    console.timeEnd('[JS] Rust Wasm total execution');
    console.log(`[JS] Rust Wasm call returned successfully.`);
    addMessage(answer, 'system');
  } catch (err: any) {
    addMessage(`Error from Rust Logic: ${err.message || err}`, 'system');
  } finally {
    setLoading(false);
  }
}

sendBtn.addEventListener('click', handleSend);
chatInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') handleSend();
});

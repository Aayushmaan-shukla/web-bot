chrome.runtime.onInstalled.addListener(() => {
  console.log("Web AI Bot Extension installed.");
  chrome.sidePanel.setPanelBehavior({ openPanelOnActionClick: true }).catch((error) => console.error(error));
});

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === "EXTRACTED_CONTENT") {
    console.log("Received content from page:", request.payload);
    // Future logic: Send to Wasm UI or LLM API
  }
  return true;
});

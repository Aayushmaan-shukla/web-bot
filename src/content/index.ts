console.log("Web AI Bot Content Script injected.");

// Basic function to extract text
function extractPageContent() {
  // Simple extraction: grab all text or specific blocks
  const mainText = document.body.innerText;
  return mainText.substring(0, 5000); // Limit for now
}

// Listen for messages from the sidepanel
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === "GET_CONTEXT") {
    sendResponse({ payload: extractPageContent() });
  }
  return true; // Keep the message channel open for async responses if needed
});

// console.log('This is a popup!');
// const button = document.createElement('button');
// const input = document.createElement('input');
// button.textContent = 'Click me!';
// button.addEventListener('click', () => {
//   input.value = 'Hello, world!';
//   console.log('Button clicked!');
// });
chrome.runtime.onInstalled.addListener(() => {
  chrome.action.setBadgeText({
    text: "OFF",
  });
});
// chrome.runtime.onConnect.addListener(function(port) {
//   if (port.name === "popup") {
//     port.postMessage({ fromBackground: 'Hello from background!' });
//   }
// });
// background.js
chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
  if (request.data) {
    console.log('Message from popup:', request.data);
    // chrome.tabs.query({ active: true, currentWindow: true }, function(tabs) {
    //   const activeTab = tabs[0];
    //   // 向当前活动标签页发送消息
    //   chrome.tabs.sendMessage(activeTab.id, { data: 'Hello from background' });
    // });
  }
});
console.log('background.js')
// chrome.tabs.query({ active: true, currentWindow: true }, function(tabs) {
//   const activeTab = tabs[0];
//   // 向当前活动标签页发送消息
//   chrome.tabs.sendMessage(activeTab.id, { data: 'Hello from background' });
// });

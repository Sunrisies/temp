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
// 在谷歌插件里面怎么获取input的值
// chrome.storage.sync.get(['inputValue'], function(result) {
//   console.log(result.inputValue);
// });
// background.js
console.log(chrome,'1=1=1==1',self)
chrome.action.onClicked.addListener((tab) => {
  // 当扩展图标被点击时，执行下面的代码
  console.log("扩展图标被点击了！",self);

  // 可以在这里执行各种操作，例如打开新的标签页，注入脚本等
  // 例如：注入content script到当前标签页
  /* chrome.scripting.executeScript({
    target: { tabId: tab.id },
    files: ['content.js']
  }); */

  // 打开一个新的标签页
  /* chrome.tabs.create({ url: 'https://example.com' }); */
});

// 获取里面的点击事件
// chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
//   console.log(request.message);
//   if (request.message === 'clicked_browser_action') {
//     console.log('Browser action clicked!');
//     sendResponse({ message: 'Browser action clicked!' });
//   }
// });
// document.body.appendChild(button);
// document.body.appendChild(input);
// document.body.appendChild(button);





// {
//   "name": "Hello Extensions",
//   "description": "Base Level Extension",
//   "version": "1.0",
//   "manifest_version": 3,
//   "permissions": [
//     "activeTab",
//     "storage",
//     "tabs"
//   ],
//   "action": {
//     "default_popup": "popup.html",

//     "default_icon": "hello_extensions.png"
//   },
//   "background": {
//     "service_worker": "background.js"
//   },
//   "content_scripts": [
//     {
//       "js": [
//         "popup.js"
//       ],
//       "matches": [
//         "https://developer.chrome.com/docs/extensions/*",
//         "https://developer.chrome.com/docs/webstore/*",
//         "https://www.baidu.com/*",
//         "https://juejin.cn/*"
//       ]
//     }
//   ]

// }
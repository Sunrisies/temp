// document.addEventListener('DOMContentLoaded', (event) => {
//   console.log('DOM fully loaded and parsed');
//   console.log('Hello, world!', document);
//   // const listItem = document.querySelector('.item');
//   // 获取li的class为item的元素

//   setTimeout(() => {
//     // 获取所有的 'content-wrapper' 元素
//     const contentWrappers = document.getElementsByClassName('content-wrapper');

//     // 遍历每个 'content-wrapper' 元素
//     for (let i = 0; i < contentWrappers.length; i++) {
//       const wrapper = contentWrappers[i];

//       // 获取标题链接和文本
//       const titleLink = wrapper.querySelector('.title-row .jj-link.title');
//       const titleText = titleLink ? titleLink.textContent : '';

//       // 获取摘要链接和文本
//       const abstractLink = wrapper.querySelector('.abstract .jj-link');
//       const abstractText = abstractLink ? abstractLink.textContent : '';

//       // 获取用户信息链接和文本
//       const userLink = wrapper.querySelector('.user-message');
//       const userText = userLink ? userLink.textContent.trim() : '';

//       // 获取浏览次数
//       const viewCountElem = wrapper.querySelector('.view span');
//       const viewCount = viewCountElem ? parseInt(viewCountElem.textContent, 10) : 0;

//       // 获取点赞次数
//       const likeCountElem = wrapper.querySelector('.like span');
//       const likeCount = likeCountElem ? parseInt(likeCountElem.textContent, 10) : 0;

//       // 获取图片的 src 属性
//       const thumbImg = wrapper.querySelector('.thumb');
//       const thumbSrc = thumbImg ? thumbImg.src : '';

//       // 打印获取的数据
//       console.log(`Title: ${titleText}`);
//       console.log(`Abstract: ${abstractText}`);
//       console.log(`User: ${userText}`);
//       console.log(`Views: ${viewCount}`);
//       console.log(`Likes: ${likeCount}`);
//       console.log(`Thumbnail Image URL: ${thumbSrc}`);
//     }
//     // const listItems = document.getElementsByClassName('content-wrapper');
//     // console.log(listItems, '=1=1=');
//     // listItems.forEach(element => {
//     //   console.log(element);
//     //   const titleLink = element.querySelector('.title');
//     //   const titleText = titleLink ? titleLink.textContent : '';
//     //   console.log(titleText);
//     // });
//   }, 3000)
//   // // 获取标题链接和文本
//   // 现在可以安全地访问和操作DOM了

//   // const titleLink = listItem.querySelector('.title');
//   // const titleText = titleLink ? titleLink.textContent : '';

//   // // 获取摘要链接和文本
//   // const abstractLink = listItem.querySelector('.abstract .jj-link');
//   // const abstractText = abstractLink ? abstractLink.textContent : '';

//   // // 获取用户信息链接和文本
//   // const userLink = listItem.querySelector('.user-message');
//   // const userText = userLink ? userLink.textContent.trim() : '';

//   // // 获取浏览次数
//   // const viewCount = listItem.querySelector('.view span');
//   // const viewCountText = viewCount ? viewCount.textContent : '';

//   // // 获取点赞次数
//   // const likeCount = listItem.querySelector('.like span');
//   // const likeCountText = likeCount ? likeCount.textContent : '';

//   // // 打印获取的数据
//   // console.log('Title:', titleText);
//   // console.log('Abstract:', abstractText);
//   // console.log('User:', userText);
//   // console.log('Views:', viewCountText);
//   // console.log('Likes:', likeCountText);
// });
// console.log('content.js',chrome.runtime)
// chrome.runtime.sendMessage({
//   data: 'Some data from content script'
// });
// chrome.runtime.sendMessage({ data: '内容脚本的数据' });
// content.js
console.log('content.js', chrome.runtime)
chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
  console.log('Message received in content script:', request, sender, sendResponse);
  if (request.data) {

    // console.log('Message received in content script:', request.data);
    // sendResponse({ response: 'Content script responded' })
    // chrome.runtime.sendResponse({ response: 'Content script responded' });
    // 根据消息执行相应操作
  }
});
// chrome.runtime.onConnect.addListener( (port) => {
//   if (port.name === "popup") {
//     console.log("Connected to popup");
//     // 处理连接...
//   }
// });
// chrome.runtime.onConnect.addListener((port) => {
//   console.log('Connected to background script');
//   // if (port.name === "popup") {
//   //   port.postMessage({ fromBackground: 'Hello from background!' });
//   // }
// });
// chrome.storage.local.set({ 'myData': 'Some data from content script' });
const getData = (id) => {
  const contentWrappers = document.getElementsByClassName(id);
  const data = [];
  // 遍历每个 'content-wrapper' 元素
  for (let i = 0; i < contentWrappers.length; i++) {
    const wrapper = contentWrappers[i];

    // 获取标题链接和文本
    const titleLink = wrapper.querySelector('.title-row .jj-link.title');
    const titleText = titleLink ? titleLink.textContent : '';

    // 获取摘要链接和文本
    const abstractLink = wrapper.querySelector('.abstract .jj-link');
    const abstractText = abstractLink ? abstractLink.textContent : '';

    // 获取用户信息链接和文本
    const userLink = wrapper.querySelector('.user-message');
    const userText = userLink ? userLink.textContent.trim() : '';

    // 获取浏览次数
    const viewCountElem = wrapper.querySelector('.view span');
    const viewCount = viewCountElem ? parseInt(viewCountElem.textContent, 10) : 0;

    // 获取点赞次数
    const likeCountElem = wrapper.querySelector('.like span');
    const likeCount = likeCountElem ? parseInt(likeCountElem.textContent, 10) : 0;

    // 获取图片的 src 属性
    const thumbImg = wrapper.querySelector('.thumb');
    const thumbSrc = thumbImg ? thumbImg.src : '';
    data.push({ titleText, abstractText, userText, viewCount, likeCount, thumbSrc })
    // 打印获取的数据
    // console.log(`Title: ${titleText}`);
    // console.log(`Abstract: ${abstractText}`);
    // console.log(`User: ${userText}`);
    // console.log(`Views: ${viewCount}`);
    // console.log(`Likes: ${likeCount}`);
    // console.log(`Thumbnail Image URL: ${thumbSrc}`);
  }
  return data

}
document.addEventListener('DOMContentLoaded', (event) => {
  console.log('DOM fully loaded and parsed');
  console.log('Hello, world!', document);
  chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
    console.log('Message received in content script:', request, sender, sendResponse);
    if (request.data) {
      const list = getData(request.data)
      console.log(list)
      // console.log('Message received in content script:', request.data);
      sendResponse({ response: list })
      // chrome.runtime.sendResponse({ response: 'Content script responded' });
      // 根据消息执行相应操作
    }
  });
});

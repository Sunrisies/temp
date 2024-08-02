// console.log('This is a popup!');
// const button = document.getElementById('btn');
// const input = document.getElementById('input');
// button.textContent = 'Click me!';
// button.addEventListener('click', () => {
//   input.value = 'Hello, world!';
//   console.log('Button clicked!');
// });
// document.addEventListener('DOMContentLoaded', function() {
//   var clickMeButton = document.getElementById('clickMe');
//   clickMeButton.addEventListener('click', function() {
//     alert('Button clicked!');
//   });
// });
// 更改滤镜
document.getElementById('changeFilterBtn').addEventListener('click', () => {
  // alert(document.documentElement.style);
  const d = document.createElement('div');
  d.innerHTML = 'Hello, world!';
  // d.style.filter = 'hue-rotate(180deg)';
  document.body.appendChild(d);
  document.documentElement.style.filter = "hue-rotate(180deg)"
  // chrome.tabs.executeScript({
  //   code: 'document.documentElement.style.filter = "hue-rotate(180deg)"'
});
 
//去除滤镜
document.getElementById('resetFilterBtn').addEventListener('click',  () => {
  // chrome.tabs.executeScript({
  //   code: `document.documentElement.style.filter = "none"`
  // });
  document.documentElement.style.filter = "none"
});
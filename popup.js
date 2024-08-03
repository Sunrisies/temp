const resetFilterBtn = document.getElementById('searchBtn');
resetFilterBtn.onclick = async function () {
  const filterInput = document.getElementById('filterInput');
  console.log(filterInput.value)
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  console.log('tab', tab)
  const { response } = await chrome.tabs.sendMessage(tab.id, { data: filterInput.value });
  console.log('response', response)
  document.documentElement.style.backgroundColor = "red"
  document.documentElement.style.filter = "none"
  // 获取到数据添加到页面上面
  // const data = await chrome.storage.local.get('myData');
  // const div = document.createElement('div');
  // console.log('response', JSON.stringify(response.response.slice(0, 3)))
  // div.innerHTML = JSON.stringify(response);
  // createTable(data)
  document.body.appendChild(createTable(response));
}

// 创建表格
function createTable(data) {
  // 创建 table 元素
  const table = document.createElement('table');
  table.setAttribute('id', 'dynamic-table'); // 设置一个 ID 用于 CSS 选择器

  // 创建 thead 元素
  const thead = document.createElement('thead');
  table.appendChild(thead);

  // 创建 tbody 元素
  const tbody = document.createElement('tbody');
  table.appendChild(tbody);

  // 创建表头
  const headerRow = document.createElement('tr');
  ['User', 'Views', 'Likes', 'Title'].forEach(text => {
    const th = document.createElement('th');
    th.textContent = text;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);

  // 填充表格数据
  data.forEach(item => {
    const row = document.createElement('tr');
    Object.values(item).slice(0, 4).forEach(text => {
      const cell = document.createElement('td');
      cell.textContent = text;
      row.appendChild(cell);
    });
    tbody.appendChild(row);
  });

  // 添加样式
  addStyles();

  return table;
}

// 动态添加样式到页面
function addStyles() {
  const style = document.createElement('style');
  style.type = 'text/css';
  style.innerHTML = `
    #dynamic-table, #dynamic-table th, #dynamic-table td {
      border: 1px solid black;
      border-collapse: collapse;
    }
    #dynamic-table th, #dynamic-table td {
      padding: 8px;
      text-align: left;
    }
    #dynamic-table th {
      background-color: #f2f2f2;
    }
    #dynamic-table tr:nth-child(even) {
      background-color: #f9f9f9;
    }
  `;
  document.head.appendChild(style);
}

// 将表格添加到页面中
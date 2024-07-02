const fs = require('fs');

const bookmarksFile = fs.readFileSync('book.html', 'utf8');


// 使用正则表达式匹配所有的 <DT><A> 标签
const matches = bookmarksFile.match(/<DT><A HREF="([^"]+)"[^>]*>([^<]+)<\/A>/g);

if (matches) {
    const jsonArray = matches.map(match => {
        const hrefMatch = match.match(/HREF="([^"]+)"/);
        const textMatch = match.match(/>([^<]+)</);

        if (hrefMatch && textMatch) {
            return {
                name: textMatch[1],
                url: hrefMatch[1]
            };
        }
    }).filter(Boolean); // 过滤掉 undefined 的值
    const jsonContent = JSON.stringify(jsonArray, null, 2);
    fs.writeFile('output.json', jsonContent, 'utf8', (err) => {
        if (err) {
            console.error('写入文件时发生错误:', err);
        } else {
            console.log('数据已成功写入 output.json 文件');
        }
    });
    console.log(JSON.stringify(jsonArray, null, 2));
} else {
    console.log("未找到匹配的内容");
}
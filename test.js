const http = require('http');

const server = http.createServer((req, res) => {
  // 获取IP地址，注意这可能会因为代理等原因而有所不同  
  // 对于大多数直接连接，req.socket.remoteAddress会工作  
  // 但如果使用了反向代理，可能需要从HTTP头中读取  
  const ip = req.socket.remoteAddress;
  console.log(`Client IP: ${ip}`);
  const ipv4 = extractIPv4FromIPv6Mapped(ip);
  console.log(ipv4); // 输出: 180.98.153.33

  // 发送响应  
  res.writeHead(200, { 'Content-Type': 'text/plain' });
  res.end(`Hello World\n Your IP is ${ipv4}`);
});

server.listen(9012, () => {
  console.log('Server running at http://localhost:9012/');
});
function extractIPv4FromIPv6Mapped(ipv6) {
  // 检查是否是以'::ffff:'开头的IPv6映射地址  
  if (ipv6.startsWith('::ffff:')) {
    // 移除'::ffff:'前缀，并返回剩余的字符串  
    return ipv6.substring(7);
  }
  // 如果不是IPv6映射的IPv4地址，则返回null或适当的错误信息  
  return null; // 或者抛出一个错误  
}

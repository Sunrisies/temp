# 提取字符串的方法
- # slice()
```js
"use strict";

//string.slice() 提取字符串的某个部分，并以新的字符串返回被提取的部分
//方法: 可通过指定的开始和结束位置,参数1必选,可以是正数,也可以是负数,正数从左往右,负数从右往左
//参数2可选,规定从什么地方结束,从下标开始
let str = "0123456789";
let result = str.slice(2);
let result1 = str.slice(-2);
let result2 = str.slice(1, 5);
let result3 = str.slice(1, -3);
let result4 = str.slice(2, -2)
console.log('原字符串:', str) //0123456789
console.log("从索引为2的字符起一直到结束:", result);  //23456789
console.log("从倒数第2个字符起一直到结束:", result1);  //89
console.log("从开始一直到索引为5的前一个字符:", result2);  //1234
console.log("从开始一直到倒数第3个字符的前一个字符:", result3);  //123456
console.log("从索引为3的字符起到倒数第3个字符的前一个字符:", result4);  //234567
```

- # substring()
```js
"use strict";

//string.substring() 截取父字符串的一部分
//方法: 参数1是开始的位置,参数2是结束的位置,下标从0开始,如果是负数,会将所有的负数转成0
let str = 'The quick brown fox jumps over the lazy dog';
let result = str.substring(4, 10);
let result2 = str.substring(7,-6);
console.log(result); //quick
console.log(result2); // The qui
```
- ## 案例
> 获取指定字符最后一个之后的数据，适用于判断图片是不是某种类型
```js
"use strict";

let str = "weqwe.wqewqe.wqewqe.qwewqe.wqewqew.qwewqe.ipng";
let str1 = str.substring(str.lastIndexOf(".") + 1);
console.log(str1);//ipng
```

# 获取字符串位置的方法

- # indexOf()
```js
"use strict";

//string.indexOf() 判断一个字符是否在字符串中存在，
//从前往后查 如果存在返回该元素或字符第一次出现的位置的索引，不存在返回 - 1。从0开始
let str = 'chaoyang';
let result = str.indexOf("y")
console.log(result)//4
let result1 = str.indexOf("l")
console.log(result1)//-1
```

- # lastIndexOf()
```js
"use strict";

//string.lastIndexOf() 判断一个字符是否在字符串中存在，
//从后往前查 如果存在返回该元素或字符第一次出现的位置的索引，不存在返回 - 1。下标是0开始
let str = 'abababcdsad';
let result1 = str.lastIndexOf("d")
console.log(result1)//10
let result2 = str.lastIndexOf("l")
console.log(result2)//-1
```

- # search()
```js
"use strict";

// string.search() 查找某个字符在字符串中的位置，
//方法: 如果没有找到就返回-1,找到就返回下标,从0开始,可以使用正则,效果跟indexOf()类似，但是没有他好用
let filter = 'String';
let result = "SourceString".search('/' + filter + '/i');
console.log(result)//-1
let re = new RegExp(filter1, 'i');
let result1 = 'SourceString'.search(re);
console.log(result1)//6
```

# 判断字符串是否包含另一个字符串

- # startsWith()
```js
"use strict";

// string.startsWith() 检测字符串是否以指定的子字符串开始,返回结果是true或者false
let str = 'The quick brown fox jumps over the lazy dog';
let result = str.startsWith('The');
console.log(result) //true
let result1 = str.startsWith('the')
console.log(result1) //false
```

- # endsWith()
```js
"use strict";

// string.endsWith() 检测字符串是否以指定的子字符串结束,返回结果是true或者false
let str = 'The quick brown fox jumps over the lazy dog';
let result = str.endsWith('dog');
console.log(result) //true
let result1 = str.endsWith('Dog')
console.log(result1) //false
```

- # includes()
```js
"use strict";

//string.includes() 判断字符串是否包含指定的子串
//包含返回true,不包含返回false
let str = 'do not worry be happy';
let result1 = str.includes("be")
console.log(result1)//true
let result2 = str.includes("bes")
console.log(result2)//false
```
# 删除空格

- # trim()
```js
"use strict";

// string.trim() 删除左右空格
let text = "       Hello World!        ";
let result = text.trim();
console.log(result); // 'Hello World!'
```


- # trimRight()
```js


```
- # trimEnd()
- 昵称从一个其他的地方可以看到的字符。
```js 
"use strict";

// string.trimEnd() 删除右侧的空格
let str = '   foo  ';
console.log(str.length); // 8
str = str.trimEnd();
console.log(str.length); // 6
console.log(str);        // '   foo'
```

- # trimStart()
```js
"use strict";

// string.trimStart(); 删除左侧的空格
let str = '   foo  ';
console.log(str.length); // 8
str = str.trimStart();
console.log(str.length); // 5
console.log(str);        // 'foo  '
```

# 复制整个字符串

- # repeat()
```js
"use strict";

//string.repeat() 重复字符串
//方法: 参数是复制的次数
let str = "qwe"
let result = str.repeat(2)
console.log(result) //qweqwe
```

# 复制字符串

- # padStart()
```js
"use strict";

//string.padStart() 某个字符串不够指定长度，会在头部补全
//方法: 第一个参数用来指定字符串的最小长度，第二个参数是用来补全的字符串,
//如果原字符串的长度，等于或大于指定的最小长度，则返回原字符串。
let str = 'xx';
let result = str.padStart(5, '*');
console.log(result) //***xx
let result1 = str.padStart(2, '*')
console.log(result1) //xx
```
 
- # padEnd()
```js
"use strict";

//string.padEnd() 某个字符串不够指定长度，会在尾部补全
//方法: 第一个参数用来指定字符串的最小长度，第二个参数是用来补全的字符串
//如果原字符串的长度，等于或大于指定的最小长度，则返回原字符串。
let str = 'xx';
let result = str.padEnd(5, '*');
console.log(result) //xx***
let result1 = str.padEnd(2, '*')
console.log(result1) //xx
```

# 字符串的大小转换

- # 字母大小写转换
```js
"use strict";

// string.toLocaleLowerCase() 使字母变成小写
//string.toLocaleUpperCase() 使字母变成大写
var str = "Hello World";
let result = str.toLocaleLowerCase();
console.log(result); //hello world
let result1 = str.toLocaleUpperCase();
console.log(result1); //HELLO WORLD
```

# 字符串的匹配替换
- # replace()
```js
"use strict";

// string.replace() 替换字符串中的字段 ，返回一个新字符串，原字符串不会改变
//方法: 参数1是被替换的字符串，参数2是要替换的字符串,只会替换第一个出现的字符串
let str = '我爱北京，北京是一个文明省份'
let result = str.replace('北京', '河南');
console.log(str) //我爱北京，北京是一个文明省份
console.log(result)//我爱河南，北京是一个文明省份
```

- # replaceAll()
```js
"use strict";

// string.replaceAll() 替换字符串中的字段 ，返回一个新字符串，原字符串不会改变
//方法: 参数1是被替换的字符串，参数2是要替换的字符串，替换字符串中出现的所有字段
let str = '我爱北京，北京是一个文明省份'
let result = str.replaceAll('北京', '河南');
console.log(str) //我爱北京，北京是一个文明省份
console.log(result)//我爱河南，河南是一个文明省份
```

- # split()
```js
"use strict";

// string.split()  分割字符串
//方法: 参数1通过搜索字符串找到跟这个参数一样得字符,从这些字符开始进行分割,分别把分割得数据放进一个数组中,必填
//参数2是返回得数据有多少个,可选
let str = 'The quick brown fox jumps over the lazy dog';
let result1 = str.split(' ');
console.log(result1); // ['The', 'quick', 'brown', 'fox', 'jumps', 'over', 'the', 'lazy', 'dog']
let result2 = str.split(' ', 3);
console.log(result2) //['The', 'quick', 'brown']
```

- # at()
```js
"use strict";

//string.at(index) 用来查询某个字段
//方法:访问index参数处的元素 从0开始
// let str = 'chaoyang';
// let result = str.at(0)
// console.log(result)//c
// let result1 = str.at(1)
// console.log(result1)//h
// let result2 = str.at(-1)
// console.log(result2)//g
```

- # charAt()
```js
"use strict";

//string.charAt(index) 用来查询某个字段
//方法: 可返回index处的元素。从0开始
// let str = 'chaoyang';
// let result = str.charAt(0)
// console.log(result)//c
// let result1 = str.charAt(1)
// console.log(result1)//h
// let result2 = str.charAt(5)
// console.log(result2)//a
```

- # concat()
```js
"use strict";

//string.concat() 用来拼接字符串
//方法: 不会改变str和str1自身的值，返回一个新的字符串；且可支持一次性拼接多个字符串。
let str = "abc";
let str1 = "123";
let result = str.concat(str1);
let result1 = str.concat(str, str1);
console.log(result); //abc123
console.log(result1); //abcabc123
console.log(str); //abc
console.log(str1); //123
```









- # toString()
```js
"use strict";

// string.toString() 把数据转成字符串格式
let str = 30;
let result = str.toString();
console.log(typeof result); // '30'
```
对象的**`toString()`** 方法返回一个字符串，表示指定的字符串。


































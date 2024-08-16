# 判断是不是数组
- # Array.isArray()  instanceof
```js
"use strict";

//Array.isArray(value)静态方法确定传递的值是否为Array. 
let arr = [];
let arr1 = {};
let results1 = Array.isArray(arr);
let results2 = Array.isArray(arr1);
console.log(results1, results2); // true false
console.log(arr instanceof Array) // true
console.log(arr1 instanceof Array) //false
```

# 数组迭代器
> keys() 返回数组索引的迭代器 values() 返回数组元素的迭代器 values() 返回索引/值对的迭代器
```js
"use strict"

// keys() 返回数组索引的迭代器 values() 返回数组元素的迭代器 values() 返回索引/值对的迭代器
// 通过 Array.from() 直接转成数组实例
let a = ['foo', 'bar', 'baz', 'qux'];
const akeys = Array.from(a.keys());
const aValues = Array.from(a.values());
const aEntries = Array.from(a.entries());
console.log(akeys);
console.log(aValues);
console.log(aEntries);

//使用SE6的解构可以得到键跟值
for (const [index,element] of a.entries()) {
  console.log(index, element);
}
```
![image.png](http://blog.chaoyang1024.top:9000/images/2ebe9d35-ec4e-4fa4-8827-290b6d508b1e.png)

- # array.from()
```js
"use strict"

//Array.from(arrayLike,mapFn,thisArg) 方法对一个类似的方法或可重复创建一个新的，浅拷贝的材料实例。
//arrayLike 要循环的数组 mapFn 数组的每一项
let arr = [1, 2, 3, 4]
let result = Array.from(arr, (x) => x + x)
console.log(arr) // [1, 2, 3, 4]
console.log(result) // [2, 4, 6, 8]
```
![image.png](http://blog.chaoyang1024.top:9000/images/c9063e38-2e3f-43a4-927c-a71de2ef9708.png)

# 复制跟填充
- # array.fill()
```js
"use strict"

//Array.fill(value,start,end) 方法使用索引中的一个索引从一个索引到一个索引中的所有索引中止
//value填充数组的值,start开始填充的地方包括在内(可选),end结束不包括(可选)
//会改变原来数组
let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let arr3 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let result = arr1.fill(4)
console.log(arr1) // [4, 4, 4, 4, 4, 4, 4, 4, 4, 4]
console.log(result) // [4, 4, 4, 4, 4, 4, 4, 4, 4, 4]
let result2 = arr2.fill(2, 1)
console.log(result2) // [1, 2, 2, 2, 2, 2, 2, 2, 2, 2]
let result3 = arr3.fill(5, 3, 6)
console.log(result3) // [1, 2, 3, 5, 5, 5, 7, 8, 9, 10]
```
- # array.copyWithin()
```js
"use strict"

// copyWithin() 按照指定范围复制数据,插入到索引开始的位置
let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let results1 = a.copyWithin(3);
console.log(results1); //) [1, 2, 3, 1, 2, 3, 4, 5, 6, 7]
let results2 = a.copyWithin(0, 3); // 从索引3开始复制,插入到索引0开始的地方
console.log(results2); //) [4, 5, 6, 7, 8, 9, 10, 8, 9, 10]
let results3 = a.copyWithin(4, 7, 8); //索引从7开始复制,插入到索引4的地方到索引8结束
console.log(results3); //)  [1, 2, 3, 4, 8, 6, 7, 8, 9, 10]
```

# 数组转换方法
> toLocaleString()返回以','隔开的字符串 toString()返回以','隔开的字符串 valueOf()返回本身, join()接收一个参数，即字符串分隔符
```js
"use strict"

// toLocaleString()返回以','隔开的字符串 toString()返回以','隔开的字符串 valueOf()返回本身, join()接收一个参数，即字符串分隔符,默认是','
let colors = ['red', 'green', 'blue'];
console.log(colors.toLocaleString()); // red,green,blue
console.log(colors.toString()); // red,green,blue
console.log(colors.valueOf()); // ['red', 'green', 'blue']
console.log(colors.join('*')); // red*green*blue
```

# 添加或者删除方法
- # array.pop()
 ```js
"use strict"

//Array.pop() 该方法 从数组中删除最后一个元素并返回该元素。此方法更改数组的长度。会改变原来数组
let arr = [1, 2, 3, 4, 5, 6]
let result = arr.pop()
console.log(arr) //  [1, 2, 3, 4, 5]
console.log(result) // 6
```

- # array.push()
```js
"use strict"

// Array.push() 方法将一个或多个元素添加到数组的末尾并返回数组的新长度。会改变原来数组
let arr = [1, 2, 3, 4, 5]
let result = arr.push(6);
console.log(result) // 6
console.log(arr) // [1, 2, 3, 4, 5, 6]
```

- # array.shift()

```js
"use strict"

//Array.shift() 从数组中删除第一个元素并返回该删除的元素。此方法更改数组的长度。会改变原数组
let arr = [1, 2, 3, 4, 5]
let result = arr.shift()
console.log(arr) // [2, 3, 4, 5]
console.log(result) // 1
```

- # array.unshift()

```js
"use strict"

//Array.unshift() 在数组前面添加元素。此方法更改数组的长度。会改变原数组
let arr = [1, 2, 3, 4, 5]
let result = arr.unshift(0)
console.log(arr) // [0, 1, 2, 3, 4, 5]
console.log(result) // 6
```

# 数组排序
- # array.sort()

```js
"use strict"

// sort() 第一个参数减第二个参数从小往大排,第二个参数减第一个参数从大往小拍,会改变原数组
let values = [13, 22, 3, 34, 55];
let results = values.sort((a, b) => b - a);
console.log(values); // [55, 34, 22, 13, 3]
console.log(results); // [55, 34, 22, 13, 3]
```

- # array.reverse()

```js
"use strict"

// reverse() 把数组倒序,会改变原数组
let values = [13, 22, 3, 34, 55];
let results = values.reverse();
console.log(values) // [55, 34, 3, 22, 13]
console.log(results); // [55, 34, 3, 22, 13]
```

# 数组的操作方法
- # array.concat()
```js
"use strict"

//Array.concat(array) 这个方法不会改变现有的种类，不同的返回一个新的种类。
let arr = [1, 2, 3, 4, 5, 6, 7, 8]
let arr2 = [1, 2, 3, 4, 5, 6]
console.log(arr) //[1, 2, 3, 4, 5, 6, 7, 8]
console.log(arr2) // [1, 2, 3, 4, 5, 6]
let result = arr.concat(arr2)
console.log(result) // [1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6]
//深拷贝数组
let result2 = arr.concat()
console.log(result2) // [1, 2, 3, 4, 5, 6, 7, 8]
console.log(arr === result2) // false
```

- # array.slice()
```js
"use strict"

// slice() 创建一个包含原有数组一个或者多个的新数组,第一个参数会返回该索引到数组末尾的所有元素,第二个参数结束的位置,不包含结束的这个数
let colors = ['red', 'green', 'blue', 'yellow', 'purple'];
let results1 = colors.slice(1); // 从索引为1的元素开始截取
let results2 = colors.slice(1, 4); // 从索引为1的元素开始截取,截取到索引为4的,不包含索引为4的这个元素
console.log(results1); // ['green', 'blue', 'yellow', 'purple'] 
console.log(results2); // ['green', 'blue', 'yellow']
```

- # array.splice()

```js
"use strict"

// splice() 删除:需要两个参数,第一个参数是要删除的位置,第二个参数是删除元素的数量
let colors = ['red', 'green', 'blue', 'yellow', 'purple'];
let results1 = colors.splice(0, 1); // 删除第一项
console.log(results1); // ['red']
console.log(colors) // ['green', 'blue', 'yellow', 'purple']
// 插入:需要三个参数,第一个参数是开始位置,第二个参数是要删除的元素,第三个参数是添加的元素,可以说多个
let colors1 = ['green', 'blue', 'yellow', 'purple'];
let results2 = colors1.splice(1, 0, 'red'); // 从索引1开始删除两个元素,然后在从索引1开始插入元素
console.log(colors1) // ['green', 'red', 'blue', 'yellow', 'purple']
console.log(results2); // []
// 替换:删除元素的同时插入指定位置的新元素,需要三个参数,第一个参数是开始位置,第二个参数是要删除的元素,第三个参数是添加的元素,可以说多个
let colors2 = ['red', 'green', 'blue', 'yellow', 'purple'];
let results3 = colors2.splice(1, 2, 'blakc'); // 从索引1开始删除两个元素,然后在从索引1开始插入元素
console.log(colors2); // ['red', 'blakc', 'yellow', 'purple']
console.log(results3); // ['green', 'blue']
// 插入跟替换的区别是插入时要删除的元素为0,替换的时候不是0
```

# 搜索和位置的方法
## 严格相等
- # array.includes()
```js
"use strict"

//Array.includes(valueToFind,fromIndex) 方法确定数组是否在其条目中包含某个值，返回true或 false酌情。
//valueToFind要查找的值,formindex开始查找的下标,默认0
let arr = [1, 2, 3, 4, 5]
let result = arr.includes(3)
console.log(result) //true
let result1 = arr.includes(3, 4)
console.log(result1) //false
let result2 = arr.includes(8)
console.log(result2) //false
```

- # array.indexOf()
```js
"use strict"

//Array.indexOf(searchElement,fromIndex) 方法返回可以在数组中找到给定元素的第一个索引，如果不存在，则返回 -1。默认是从0开始
let arr = ['ant', 'bison', 'camel', 'duck', 'bison'];
let result = arr.indexOf('camel')
console.log(result) // 2
let result1 = arr.indexOf('11')
console.log(result1) // -1
```

- # array.lastIndexOf()
```js
"use strict"

//Array.lastIndexOf(searchElement,fromIndex) 方法返回可以在数组中找到给定元素的最后一个索引，如果不存在，则返回 -1。向后搜索数组，
// searchElement被查找的值,fromIndex是开始的位置,默认是从最后一个开始
let arr = [1, 2, 3, 4, 5, 4, 3, 2, 1]
let reduce = arr.lastIndexOf(3)
console.log(reduce) // 6
```

## 断言函数
- # array.find()
```js
"use strict"

//Array.find(callbackFn(element,index,array),thisArg) 返回方法列表中满足的测试功能的第一个元素的值。否则返回undefined。
let arr = ['spray', 'limit', 'elite', 'exuberant', 'destruction', 'present'];
let result = arr.find((element, index, arr) => {
    console.log(element) //值
    console.log(index) //key
    console.log(arr) //数组
    return element === 'exuberant' //会一直找下去,直到返回值为true,或者undefined
})
console.log(result) //limit
```

- # array.findindex()
```js
"use strict"

//Array.findIndex(callbackFn(element,index,array),thisArg) 返回方法目录则中满足的测试函数的第一个元素的索引。如果没有找到元素返回-1
let arr = ['spray', 'limit', 'elite', 'exuberant', 'elite', 'destruction', 'present'];
let result = arr.findIndex((element, index, arr) => {
    console.log(element) //值
    console.log(index) //key
    console.log(arr) //数组
    return element === 'elite' // 2  会一直找下去,直到返回值为true,或者没有找到就直接返回-1
    // return element === '11' // -1
})
console.log(result)
```

- # array.findLast()
```js
"use strict"

// Array.findLast(callbackFn(element,index,array),thisArg) 返回方法中最终满足提供的测试函数条件的元素的值。如果没有找到合适的元素，返回undefined。
let arr = ['spray', 'limit', 'elite', 'exuberant', 'elite', 'destruction', 'present'];
let result = arr.findLast((element, index, arr) => {
    console.log(element) //值
    console.log(index) //key
    console.log(arr) //数组
    return element === 'exuberant' // 会一直找下去,直到返回值为true,或者没有找到就直接返回undefined,从后往前找
})
console.log(result) //exuberant
```

![image.png](http://blog.chaoyang1024.top:9000/images/8dfacd26-4887-4551-9e10-aa35af1db0ed.png)

- # array.findLastIndex()
```js 
"use strict"

//Array.findLastIndex(callbackFn(element,index,array),thisArg) 返回方法目录中最终满足提供的测试函数条件的元素。如果找不到元素，返回 -1。
let arr = ['spray', 'limit', 'elite', 'exuberant', 'elite', 'destruction', 'present'];
let result = arr.findLastIndex((element, index, arr) => {
    console.log(element) //值
    console.log(index) //key
    console.log(arr) //数组
    return element === 'exuberant' // 3 会一直找下去,直到返回值为true,或者没有找到就直接返回-1,从后往前找
})
console.log(result) //3
```

![image.png](http://blog.chaoyang1024.top:9000/images/aedf00ad-75b1-4b34-a275-2fc70aa99755.png)

# 迭代方法
- # array.every()
```js
"use strict"

//Array.every(callbackFn(element,index,arr),thisArg) 方法目录所有元素的测试是否通过指定函数内的函数返回一个布尔值。
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let result = arr.every((element, index, arr) => {
    console.log(element) //当前值
    console.log(index) //当前key
    console.log(arr) //数组
    return element > 0 //true
    // return element > 3 //false
})
console.log(result)
```

- # array.filter()
```js
"use strict"

//Array.filter(callbackFn(element,index,array),thisArg) 创建给定一些的浅拷贝方法 (en-US)，包含通过所提供的函数实现的测试的所有元素。
let arr = ['spray', 'limit', 'elite', 'exuberant', 'destruction', 'present'];
let result = arr.filter((element, index, arr) => {
    console.log(element) //值
    console.log(index) //key
    console.log(arr) //数组
    return element === 'limit' //结果为true,才能通过
})
console.log(arr) //['spray', 'limit', 'elite', 'exuberant', 'destruction', 'present']
console.log(result) //['limit']
```

- # array.forEach()
```js
"use strict"

//Array.forEach(callbackFn(element,index,array),thisArg) 方法对每种元素执行一次给定的功能。没有返回值
let arr = ['spray', 'limit', 'elite', 'exuberant', 'elite', 'destruction', 'present'];
let arr2 = []
let result = arr.forEach((element, index, arr) => {
    console.log(element) //当前值
    console.log(index) //当前key
    console.log(arr) //数组
    arr2.push(element)
})
console.log(arr2) //['spray', 'limit', 'elite', 'exuberant', 'elite', 'destruction', 'present']
console.log(result) //undefined
```

![image.png](http://blog.chaoyang1024.top:9000/images/bd1c0e25-7c47-4a42-a506-c998b8ed9265.png)

- # array.map()
```js
"use strict"

//Array.map() 方法创建一个新数组，其中填充了对调用数组中的每个元素调用提供的函数的结果。
// element当前项, index当前下标, arr数组
let arr = [1, 2, 3];
let result = arr.map((element, index, arr) => {
    console.log(element, '+', index, '+', arr)
    return element + element
});
console.log(arr) // [1, 2, 3]
console.log(result)// [2, 4, 6]
```
![image.png](http://blog.chaoyang1024.top:9000/images/9c85e087-d79f-42dd-86fe-3dcef42f33dc.png)

- # array.some()

```js
"use strict"

// some(callbackFn(element,index,arr),thisArg) 方法搜索条件中的元素的只要有一项结果是true,那么返回值就是true
// element表示key,index表示value,arr表示数组
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let result = arr.some((element, index, arr) => {
  return element > 7 //true
})
console.log(result)
```

# 归并方法
- # array.reduce()
```js
"use strict"

// reduce(）第一次执行函数:perv表示第一项(1),cur表示第二项(2),第二次执行函数:perv就是第一次执行函数结果3(1+2),cur就是第三项3,如此递归,直到最后,并还回结果
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let sum = values.reduce((prev, cur, index, array) => {
  return prev + cur;
});
console.log(sum); // 55
```

- # array.reduceRight()

```js
"use strict"

// reduceRight(）第一次执行函数:perv表示最后一项(10),cur表示倒数第二项(9),第二次执行函数:perv就是第一次执行函数结果19(10+9),cur就是倒数第三项8,如此递归,直到第一项,并还回结果
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let sum = values.reduceRight((prev, cur, index, array) => {
  return prev + cur;
});
console.log(sum); // 55
```

# 新增的方法
- # array.at()
    
```js
"use strict"

//Array.at(index)  方法接收一个整数值并返回该索引的项目，允许正数和负数。负整数从数组中的最后一个项目开始倒数。
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
let result = arr.at(-1)
let result2 = arr.at(1)
console.log(result) //11
console.log(result2) //2
```

- # array.flat()
```js
"use strict"

//Array.flat(depth) 方法会按照一个可指定的深度递归遍历数组，并将所有元素与遍历到的子数组中的元素合并为一个新数组返回。
//depth 默认是1,可以把二维数组变成一维数组,2可以把三维数组变成二维数组,Infinity把多维数组变成一维数组
let arr = [1, 2, [3, 4, [5, 6, [7, 8, [9, 10]]]]];
let result1 = arr.flat()
console.log(result1) //[1, 2, 3, 4, Array(3)]
let result2 = arr.flat(2)
console.log(result2) // [1, 2, 3, 4, 5, 6, Array(3)]
let result3 = arr.flat(Infinity)
console.log(result3) //  [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
console.log(arr) // [1, 2, Array(3)]
```
![image.png](http://blog.chaoyang1024.top:9000/images/f13c6bf3-0eb8-4d70-92c6-3867f698d5f5.png)















































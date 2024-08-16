### 1.var、let、const区别

```
var、let、const 共同点都是可以声明变量的
    区别一：
    var 具有变量提升的机制
    let和const没有变量提升的机制
    区别二：
      var 可以多次声明同一个变量
      let和const不可以多次声明同一个变量
    区别三：
      var、let声明变量的
      const声明常量
      var和let声明的变量可以再次赋值，但是const不可以再次赋值了。
    区别四：
      var声明的变量没有自身作用域
      let和const声明的变量有自身的作用域
```

### 2.作用域考题

#### 1.考题一：let和const没有变量提升性

```
console.log( str );//undefined
var str = '你好';
console.log( num );//报错
let num = 10;
```

#### 2.考题二：

```
function demo(){
  var n = 2;
  if( true ){
    var n = 1;
  }
  console.log( n );//1
}
demo();
​
function demo(){
  let n = 2;
  if( true ){
    let n = 1;
  }
  console.log( n );//2
}
demo();
```

#### 3.考题三：可以修改

```
const obj = {
  a:1
}
obj.a = 11111;
console.log( obj )
​
const arr = ['a','b','c'];
arr[0]= 'aaaaa';
console.log( arr );
```

### 3.将下列对象进行合并

#### 1.浅拷贝

```
方式一：Object.assign
const a = {a:1,b:4};
const b = {b:2,c:3};
let obj1 = Object.assign(a,b);
console.log( obj1 );
​
方式二：...
let obj2 = {...a,...b};
console.log( obj2 );
​
​
方式三：自己封装方法
function extend( target,  source ){
  for(var key in source){
    target[key] = source[key];
  }
  return target;
}
console.log( extend(a,b) );
```

#### 2.深拷贝

```
方式一：
let obj1 = {
  a:0,
  b:{
    c:0
  }
};
let obj2 = JSON.parse(JSON.stringify(obj1);
obj1.a = 1;
obj1.b.c = 1;
console.log(obj1);  //{a:1,b:{c:1}}
console.log(obj2);  //{a:0,b:{c:0}}
​
方式二：
function deepCopy(object) {
  if (!object || typeof object !== "object") return;
  let newObject = Array.isArray(object) ? [] : {};
  for (let key in object) {
    if (object.hasOwnProperty(key)) {
      newObject[key] = typeof object[key] === "object" ? deepCopy(object[key]) : object[key];
    }
  }
  return newObject;
}
let obj2 = deepCopy(obj1);
```

### 4.箭头函数和普通函数有什么区别？

```
1. this指向的问题
    箭头函数中的this只在箭头函数定义时就决定的，而且不可修改的（call、apply、bind）
    ****箭头函数的this指向定义时候、外层第一个普通函数的this
2. 箭头函数不能new（不能当作构造函数）
3. 箭头函数没有prototype
4. 箭头函数没有arguments
​
```

### 5.Promise有几种状态

```
有三种状态：
  pending（进行中）
  fulfilled（已成功）
  rejected（已失败）
```

### 6.setTimeout、Promise、Async/Await 的区别

```
setTimeout: setTimeout 的回调函数放到宏任务队列里，等到执行栈清空以后执行；
​
Promise: Promise本身是同步的立即执行函数，当在executor中执行resolve或者reject的时候，此时是异步操作，会先执行then/catch等，当主栈完成时，才会去调用 resolve/reject 方法中存放的方法。 
​
async: async函数返回一个Promise对象，当函数执行的时候，一旦遇到 await 就会先返回，等到触发的异步操作完成，再执行函数体内后面的语句。可以理解为，是让出了线程，跳出了 async 函数体。   
```

### 7.常用的正则表达式有哪些？

```
1.匹配 16 进制颜色值 
var regex = /#([0-9a-fA-F]{6}|[0-9a-fA-F]{3})/g; 
​
2.匹配日期，如 yyyy-mm-dd 格式 
var regex = /^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$/; 
​
3.匹配 qq 号 
var regex = /^[1-9][0-9]{4,10}$/g; 
​
4.手机号码正则 
var regex = /^1[34578]\d{9}$/g; 
​
5.用户名正则 
var regex = /^[a-zA-Z$][a-zA-Z0-9_$]{4,16}$/; 
​
```

### 8.类数组

```
一个拥有 length 属性和若干索引属性的对象就可以被称为类数组对象，类数组对象和数组类似，但是不能调用数组的方法。常见的类数组对象有 arguments 和 DOM 方法的返回结果，还有一个函数也可以被看作是类数组对象，因为它含有 length 属性值，代表可接收的参数个数。
常见的类数组转换为数组的方法有这样几种：
（1）通过 call 调用数组的 slice 方法来实现转换
  Array.prototype.slice.call(arrayLike);
（2）通过 call 调用数组的 splice 方法来实现转换
  Array.prototype.splice.call(arrayLike, 0);
（3）通过 apply 调用数组的 concat 方法来实现转换
  Array.prototype.concat.apply([], arrayLike);
（4）通过 Array.from 方法来实现转换
  Array.from(arrayLike);
```

### 9.set跟map

```
set是一种叫做集合的数据结构，map是一种叫做字典的数据结构
  什么是集合？什么又是字典？
集合：是由一堆无序的、相关联的，且不重复的内存结构【数学中称为元素】组成的组合
字典：是一些元素的集合。每个元素有一个称作key 的域，不同元素的key 各不相同
  区别？
    共同点：集合、字典都可以存储不重复的值
    不同点：集合是以[值，值]的形式存储元素，字典是以[键，值]的形式存储
```

#### 1.set

```
set是es6新增的数据结构，类似于数组，但是成员的值都是唯一的，没有重复的值，我们一般称为集合
  set本身是一个构造函数，用来生成Set数据结构
  
增删改查
  set的实例关于增删改查的方法：
    add()添加某个值，返回set结构本身当添加实例中已经存在的元素，set不会进行处理添加
      s.add(1).add(2).add(2); // 2只被添加了一次
    delete()删除某一个值，返回一个布尔值，表示删除是否成功
      s.delete(1)
    has()返回一个布尔值，判断该值是否是set的成员
      s.has(2)    
    clear()清除所有成员，没有返回值
      s.clear()
​
遍历
set实例遍历的方法有如下：
关于遍历的方法，有如下：
  ·keys()：返回键名的遍历器
  ·values()：返回键值的遍历器
  ·entries()：返回键值对的遍历器
  ·forEach()：使用回调函数遍历每个成员
set的遍历顺序就是插入顺序
keys方法、values方法、entries方法返回的都是遍历器对象
let set = new Set(['red', 'green', 'blue']);
for (let item of set.keys()) {
  console.log.(item); //red green blue
}
​
for (let item of set.values()) {
  console.log.(item); //red green blue
}
​
for (let item of set.entries()) {
  console.log.(item); //["red", "red"]   ["green", "green"]   ["blue", "blue"]
}
forEach()用于对每个成员执行某种操作，没有返回值，键值、键名都相等，同样的forEach方法有第二个参数，用于绑定处理函数的this
​
扩展运算符和 Set 结构相结合实现数组或字符串去重
// 数组
let arr = [3,4,2,3,1,12,3,4];
let unique = […new set(arr)];   //[3,4,2,1,12]
​
实现并集、交集、和差集
let a = new Set([1,2,3]);
let b = new Set([4,3,2]);
​
// 并集
let union = new Set([…a,…b]); //{1,2,3,4}
​
// 交集
let intersect = new Set([…a].filter(x => b.has(x)));  //{2,3}
​
// （a 相对于 b 的）差集
let difference = new Set([…a].filter(x => !b.has(x)));  //{1}
```

#### 2.map

```
map类型是键值对的有序列表，而键和值都可以是任意类型
  map本身是一个构造函数，用来生成Map数据结构
​
增删改查
Map结构的实例针对增删改查有以下属性和操作方法：
  ·size
    属性返回 Map 结构的成员总数。
    const map = new Map();
    map.set('foo',true);
    map.set('bar',false);
    map.size //2
  ·set()
    设置键名key对应的键值为value，然后返回整个 Map 结构
    如果key已经有值，则键值会被更新，否则就新生成该键
    同时返回的是当前Map对象，可采用链式写法
    const m = new Map();
    m.set('edition',6);
    m.set(262, 'standard');m.set(undefined,'nah')    // 键是 undefined
    m.set(1, 'a').set(2, 'b').set(3, 'c') // 链式操作
​
  ·get()
    get方法读取key对应的键值，如果找不到key，返回undefined
      const m = new Map();
      const hello = function (){console.log(‘hello’);
      m.set(hello, 'Hello ES6!');
      m.get(hello)
​
  ·has()
    has方法返回一个布尔值，表示某个键是否在当前 Map 对象之中
      const m = new Map();
      m.set('edition', 6);
      m.set(262, 'standard');
      m.set(undefined, 'nah');
      m.has('edition')     // true
      m.has('years')       // false
      m.has(262)           // true
      m.has(undefined)     // true
  
  ·delete()
    delete方法删除某个键，返回true。如果删除失败，返回false
      const m = new Map();
      m.set(undefined, 'nah');
      m.has(undefined)     // true
      m.delete(undefined)
      m.has(undefined)       // false
  ·clear()
    clear方法清除所有成员，没有返回值
      let map = new Map();
      map.set('foo', true);
      map.set('bar', false);
      map.size // 2
      map.clear()
      map.size // 0
​
遍历
  map结构原生提供三个遍历器生成函数和一个遍历方法：
  ·keys()：返回键名的遍历器
    const map = new Map([['F' ,'no'],['T',  'yes']]);
    for(let key of map.keys()){
      console.log(key)    //"F"   "T"
    }
​
  ·values()：返回键值的遍历器
    for(let key of map.values()){
      console.log(key)    //"no"    "yes"
    }
​
  ·entries()：返回所有成员的遍历器
    for(let key of map.entries()){
      console.log(key)    //"F" "no"   "T" "yes"
    };
    //或者
    for(let [key,value] of map.entries()){
      console.log(key,value)    //"F" "no"   "T" "yes"
    };
  ·forEach()：遍历 Map 的所有成员
    map.forEach(function(value,key,map){
      console.log(key,value);
    });
​
```

### 10.Fetch

特点：

```
Fetch 是一个 API，它是真实存在的，它是基于 promise 的。
使用 promise，不使用回调函数。
采用模块化设计，比如 rep、res 等对象分散开来，比较友好。
通过数据流对象处理数据，可以提高网站性能。
​
所以这里就和 Ajax 又很大不同了，一个是思想，一个是真实存在的 API，不过它们都是用来给网络请求服务的，我们一起来看看利用 Fetch 实现网络请求。
```

代码

```
<body>
  <script>
    function ajaxFetch(url) {
      fetch(url).then(res => res.json()).then(data => {
        console.info(data)
      })
    }
    ajaxFetch('https://smallpig.site/api/category/getCategory')
  </script>
</body>
```

### 11.async **和** await

```
主要考察宏任务和微任务，搭配promise，询问一些输出的顺序
原理:async 和 await 用了同步的方式去做异步，async 定义的函数的返回值都是 promise，await 后面的函数会先执行一遍，然后就会跳出整个 async 函数来执行后面js栈的代码
```

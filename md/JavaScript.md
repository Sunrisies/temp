### 1.js的数据类型

```
有两种类型
基本类型:String Number Boolean Null undefined symbol bigint
引用类型:Object Array 
NaN是一个数值类型，但是不是一个具体的数字。
```

### 2.null和undefined的区别

```
1. 作者在设计js的都是先设计的null（最初设计js的时候借鉴了java的语言）
2. null会被隐式转换成0，很不容易发现错误。
3. 先有null后有undefined，出来undefined是为了填补之前的坑。
具体区别：
    JavaScript的最初版本是这样区分的：
    null是一个表示"无"的对象（空对象指针），转为数值时为0；
    undefined是一个表示"无"的原始值，转为数值时为NaN。  
```

### 3.==和===有什么不同？

```
== : 比较的是值
    string == number || boolean || number ....都会隐式转换
    通过valueOf转换（valueOf() 方法通常由 JavaScript 在后台自动调用，并不显式地出现在代码中。）
    
=== ： 除了比较值，还比较类型
```

### 4.JS微任务和宏任务

```
1. js是单线程的语言。
2. js代码执行流程：同步执行完==》事件循环
    同步的任务都执行完了，才会执行事件循环的内容
    进入事件循环：请求、定时器、事件....
3. 事件循环中包含：【微任务、宏任务】
    微任务：promise.then
    宏任务：setTimeout..
要执行宏任务的前提是清空了所有的微任务
流程：同步==》事件循环【微任务和宏任务】==》微任务==》宏任务=》微任务...
```

### 5.JS作用域考题

```
1. 除了函数外，js是没有块级作用域。
2. 作用域链：内部可以访问外部的变量，但是外部不能访问内部的变量。
    注意：如果内部有，优先查找到内部，如果内部没有就查找外部的。
3. 注意声明变量是用var还是没有写（window.）
4. 注意：js有变量提升的机制【变量悬挂声明】
5. 优先级：声明变量 > 声明普通函数 > 参数 > 变量提升
​
面试的时候怎么看：
    1. 本层作用域有没有此变量【注意变量提升】
    2. 注意：js除了函数外没有块级作用域
    3. 普通声明函数是不看写函数的时候顺序
```

#### 1.考题一:

```
function c(){
  var b = 1;
  function a(){
    console.log( b ); //undefined
    var b = 2;
    console.log( b ); //2
  }
  a();  
  console.log( b );   //1内部可以访问外部变量，但是外部不能访问内部变量
}
c();
```

#### 2.考题二：

```
var name = 'a';
(function(){
  if( typeof name == 'undefined' ){
    var name = 'b';
    console.log('111'+name)；//打印结果是111b直接访问name是一个undefined
  }else{
    console.log('222'+name);
  }
})()
```

#### 3.考题三：

```
function fun( a ){
  var a = 10;
  function a(){}
  console.log( a ); //打印结果是：10
}
fun( 100 );
```

### 6.JS对象考题

```
JS对象注意点：
    1. 对象是通过new操作符构建出来的，所以对象之间不想等(除了引用外)；
    2. 对象注意：引用类型(共同一个地址)；
    3. 对象的key都是字符串类型；
    4. 对象如何找属性|方法；
      查找规则：先在对象本身找 ===> 构造函数中找 ===> 对象原型中找 ===> 构造函数原型中找 ===> 对象上一层原型查找
```

#### 1.考题一：

```
[1,2,3] === [1,2,3]   //false
```

#### 2.考题二：

```
var obj1 = {
  a:'hellow'
}
var obj2 = obj1;    指向的是同一个地址，其中一个改变另一个也会改变
obj2.a = 'world';
console.log(obj1);  //{a:world}
(function(){
  console.log(a);   //undefined
  var a = 1;
})();
```

#### 3.考题三：

```
var a = {}
var b = {
  key:'a'
}
var c = {
  key:'c'
}
a[b] = '123'; //向a对象里面添加b对象，
a[c] = '456'; //向a对象里面添加c对象，会覆盖b对象的内容
console.log( a[b] ); // 456
```

### 7.JS作用域+this指向+原型的考题

#### 1.考题一：

```
function Foo(){
  getName = function(){console.log(1)} //注意是全局的window.
  return this;
}
Foo.getName = function(){console.log(2)}
Foo.prototype.getName = function(){console.log(3)}
var getName = function(){console.log(4)}
function getName(){
  console.log(5)
}
​
Foo.getName();      //2，首先找foo.getName,直接打印2
getName();    //4，首先会先找到Foo里面的get，但是后面var 又重新赋值了，
Foo().getName();  //1，调用了Foo()直接找Foo函数里面的get
getName();    //1，这个时候已经找到了1
new Foo().getName();//3，new会改变this指向，这个就会向原型上面找
```

#### 2.考题二：

```
var o = {
  a:10,
  b:{
    a:2,
    fn:function(){
      console.log( this.a ); // 2
      console.log( this );   //代表b对象
    }
  }
}
o.b.fn();
```

#### 3.考题三：

```
window.name = 'ByteDance';
function A(){
  this.name = 123;
}
A.prototype.getA = function(){
  console.log( this );    //这个的this代表的window
  return this.name + 1;
}
let a = new A();
let funcA = a.getA;
funcA();  //this代表window
```

#### 4.考题四：

```
var length = 10;
function fn(){
  return this.length + 1;
}
var obj = {
  length:5,
  test1:function(){
    return fn(); //这里面的this代表的是window
  }
  test2:function fn(){  //是obj.test1 = fn;之后生成的
    return this.length + 1;
  }
}
obj.test2 = fn;             //这一步等于上面的
console.log( obj.test1() );           //11
console.log( fn()===obj.test2() );        //false
console.log( obj.test1() == obj.test2() );      //false
```

### 8.JS判断变量是不是数组，你能写出哪些方法？

#### 1.方式一：isArray

```
var arr = [1,2,3];
console.log( Array.isArray( arr ) );
```

#### 2.方式二：instanceof 【可写,可不写】

```
var arr = [1,2,3];
console.log( arr instanceof Array );
```

#### 3.方式三：原型prototype

```
var arr = [1,2,3];
console.log( Object.prototype.toString.call(arr).indexOf('Array') > -1 );
```

#### 4.方式四：isPrototypeOf()

```
var arr = [1,2,3];
console.log(  Array.prototype.isPrototypeOf(arr) )
```

#### 5.方式五：constructor。 构造函数

```
var arr = [1,2,3];
console.log(  arr.constructor.toString().indexOf('Array') > -1 )
```

### 9.js中的函数

#### 1.增

```
push()方法接收任意数量的参数，并将它们添加到数组末尾，返回数组的最新长度
                      let data = [1,2,3,4,5]
                      let count = data.push(6)
                      console.log(data) //[1,2,3,4,5,6]
                      console.log(count)  //6
​
unshift()在数组开头添加任意多个值，然后返回新的数组长度
                      let data = [1,2,3,4,5]
                      let count = data.unshift(6)
                      console.log(data)    //[6,1,2,3,4,5]
                      console.log(count)   //6
​
splice()传入三个参数，分别是开始位置、0（要删除的元素数量）、插入的元素，返回空数组
                      let colors = ["red", "green", "blue"];
                      let removed = colors.splice(1, 0, "yellow", "orange")
                      console.log(colors) // red,yellow,orange,green,blue
                      console.log(removed) // []
​
concat()创建一个新数组，把添加元素添加到新数组后面，最后返回这个新数组，不会影响原始数组
                      let data = [1,2,3,4,5]
                      let count = data.concat(6)
                      console.log(data)    //[1,2,3,4,5]
                      console.log(count)   //[1,2,3,4,5,6]
```

#### 2.删

```
pop()删除数组的最后一项，同时减少数组的 length 值，返回被删除的项
                      let data = [1,2,3,4,5]
                      let count = data.pop()
                      console.log(data)    //[1,2,3,4]
                      console.log(count)   //5
​
shift()删除数组的第一项，同时减少数组的 length 值，返回被删除的项
                      let data = [1,2,3,4,5]
                      let count = data.shift()
                      console.log(data)    //[2,3,4,5]
                      console.log(count)   //1
​
splice()传入两个参数，分别是开始位置，删除元素的数量，返回包含删除元素的数组
                      let colors = ["red", "green", "blue"];
                      let removed = colors.splice(0,1); // 删除第一项
                      console.log(colors); // green,blue
                      console.log(removed); // red，只有一个元素的数组
​
slice()创建一个新数组，在新数组上面进行删除，不会影响到原来数组，如果就一个参数，是删除该参数之前所有数据，如果有两个参数传入，截取前面那个参数+1到第二个参数之间的内容，不是下标来决定,也可以是负数
                      let colors = ["red", "green", "blue", "yellow", "purple"];
                      let colors2 = colors.slice(1);
                      let colors3 = colors.slice(1, 4);
                      console.log(colors)   // red,green,blue,yellow,purple
                      concole.log(colors2); // green,blue,yellow,purple
                      concole.log(colors3); // green,blue,yellow
```

#### 3.改

```
splice()传入三个参数，分别是开始位置，要删除元素的数量，要插入的任意多个元素，返回删除元素的数组，对原数组产生影响
                      let colors = ["red", "green", "blue"];
                      let removed = colors.splice(1, 1, "red", "purple"); // 插入两个值，删除一个元素
                      console.log(colors); // red,red,purple,blue
                      console.log(removed); // green，只有一个元素的数组
```

#### 4.查

```
indexOf()返回要查找的元素在数组中的位置,返回的是下标，如果没找到则返回-1
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      numbers.indexOf(4) // 3
​
includes()返回要查找的元素在数组中的位置，找到返回true，否则false
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      numbers.includes(4) // true
    
find()返回第一个匹配的元素
                      const people = [
                            {
                                  name: "Matt",
                                  age: 27
                            },
                            {
                                  name: "Nicholas",
                                  age: 29
                            }
                      ];
                      people.find((element, index, array) => element.age < 28) // // {name: "Matt", age: 27}
​
```

#### 5.排序

```
reverse()顾名思义，将数组元素方向排列
                      let values = [1, 2, 3, 4, 5];
                      values.reverse();
                      alert(values); // 5,4,3,2,1
​
sort()两种情况
                      let values = [0, 1, 5, 10, 15];
                      values.sort((n,m) => n - m);
                      alert(values); // 0,1,5,10,15 从小到大 第一个参数减第二个参数
​
                      let values = [0, 1, 5, 10, 15];
                      values.sort((n,m) => m - n);
                      alert(values); // 15，10，5，1，0 从大到小  第二个参数减第一个参数
```

#### 6.转换方法

```
join()方法接收一个参数，即字符串分隔符，返回包含所有项的字符串
                      let colors = ["red", "green", "blue"];
                      console.log(colors.join('=='))//red==green==blue
```

#### 7.迭代方法

```
some()  是对数组中每一项运行给定函数，如果该函数对任一项返回true，则返回true
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      let someResult = numbers.some((item, index, array) => item = 5);
                      console.log(someResult) // true
  
every() 是对数组中每一项运行给定函数，如果该函数对每一项返回true,则返回true。
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      let someResult = numbers.some((item, index, array) => item > 3);
                      console.log(someResult) // false
​
forEach()对数组每一项都运行传入的函数，没有返回值
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      numbers.forEach((item, index, array) => {
                            // 执行某些操作
                      });
​
filter()对数组每一项都运行传入的函数，函数返回true的项会组成数组之后返回
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      let someResult = numbers.filter((item, index, array) => item > 2);
                      console.log(someResult) // 3,4,5,4,3
​
map()对数组每一项都运行传入的函数，返回由每次函数调用的结果构成的数组
                      let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
                      let someResult = numbers.map((item, index, array) => item * 2);
                      console.log(someResult) // 2,4,6,8,10,8,6,4,2
```

### 10.slice是干嘛的、splice是否会改变原数组

```
1. slice是来截取的
    参数可以写slice(3)、slice(1,3)、slice(-3)
    返回的是一个新的数组
    
2. splice 功能有：插入、删除、替换
    返回：删除的元素
    该方法会改变原数组
```

### 11.JS数组去重

#### 1.方式一：new set

```
var arr1 = [1,2,3,2,4,1];
function unique(arr){
  return [...new Set(arr)]
}
console.log(  unique(arr1) );
```

#### 2.方式二：indexOf

```
var arr2 = [1,2,3,2,4,1];
function unique( arr ){
  var brr = [];
  for( var i=0;i<arr.length;i++){
    if(  brr.indexOf(arr[i]) == -1 ){
      brr.push( arr[i] );
    }
  }
  return brr;
}
console.log( unique(arr2) );
//先把数组中的元素从头添加，进行判断当返回值为1的时候，就不会添加数据到brr这个数组中，直到最后
```

#### 3.方式三：sort

```
var arr3 = [1,2,3,2,4,1];
function unique( arr ){
  arr = arr.sort();
  var brr = [];
  for(var i=0;i<arr.length;i++){
    if( arr[i] !== arr[i-1]){
      brr.push( arr[i] );
    }
  }
  return brr;
}
console.log( unique(arr3) );
//首先把数据进行从新排序，从小到大，在进行对比，拿前一个数据跟后一个数据进行对比，如果不相等，就添加到brr数组里面   
```

#### 4.方式四：reduce()

```
let arr = [1,1,2,3,4,5,5,6]
let arr2 = arr.reduce(function(ar,cur) {
  if(!ar.includes(cur)) {
    ar.push(cur)
}
  return ar
},[])
```

#### 5.方式五：filter()

```
这种方法会有一个问题:[1,'1']会被当做相同元素，最终输入[1] 
let arr = [1,1,2,3,4,5,5,6]
let arr2 = arr.filter(function(item,index) {
// indexOf() 方法可返回某个指定的 字符串值 在字符串中首次出现的位置
  return arr.indexOf(item) === index
})
```

### 12.找出多维数组最大值

```
function fnArr(arr){
  var newArr = [];
  arr.forEach((item,index)=>{
    newArr.push( Math.max(...item)  )
  })
  return newArr;
}
console.log(fnArr([
  [4,5,1,3],
  [13,27,18,26],
  [32,35,37,39],
  [1000,1001,857,1]
]));
​
​
还有一种方法
let newArr = arr.flat(Infinity)
return max = Math.max(…newArr)
```

### 13.给字符串新增方法实现功能

```
给字符串对象定义一个addPrefix函数，当传入一个字符串str时，它会返回新的带有指定前缀的字符串，例如：console.log( 'world'.addPrefix('hello') )  控制台会输出helloworld
解答：
String.prototype.addPrefix = function(str){
  return str  + this;
}
console.log( 'world'.addPrefix('hello') )
```

### 14.找出字符串出现最多次数的字符以及次数

```
var str = 'aaabbbbbccddddddddddx';
var obj = {};
for(var i=0;i<str.length;i++){
  var char = str.charAt(i);
  if( obj[char] ){
    obj[char]++;
  }else{
    obj[char] = 1;
  }
}
console.log( obj );
//统计出来最大值
var max = 0;
for( var key in obj ){
  if( max < obj[key] ){
    max = obj[key];
  }
}
//拿最大值去对比
for( var key in obj ){
  if( obj[key] == max ){
    console.log('最多的字符是'+key);
    console.log('出现的次数是'+max);
  }
}
```

### 15.new操作符具体做了什么

```
1. 创建了一个空的对象
  2. 将空对象的原型，指向于构造函数的原型
  3. 将空对象作为构造函数的上下文（改变this指向）
  4. 对构造函数有返回值的处理判断
​
  function Fun( age,name ){
    this.age = age;
    this.name = name;
  }
  function create( fn , ...args ){
    //1. 创建了一个空的对象
    var obj = {}; //var obj = Object.create({})
    //2. 将空对象的原型，指向于构造函数的原型
    Object.setPrototypeOf(obj,fn.prototype);
    //3. 将空对象作为构造函数的上下文（改变this指向）
    var result = fn.apply(obj,args);
    //4. 对构造函数有返回值的处理判断
    return result instanceof Object ? result : obj;
  }
  console.log( create(Fun,18,'张三')   )
```

### 16.闭包

```
1. 闭包是什么
    闭包就是能读取其他函数内部变量的函数。
2. 闭包可以解决什么问题【闭包的优点】
    2.1 内部函数可以访问到外部函数的局部变量
    2.2 闭包可以解决的问题
    var lis = document.getElementsByTagName('li');
    for(var i=0;i<lis.length;i++){
      (function(i){
        lis[i].onclick = function(){
          alert(i);
        }
      })(i)
    }
3. 闭包的缺点
    3.1 变量会驻留在内存中，造成内存损耗问题。
    解决：把闭包的函数设置为null
    3.2 内存泄漏【ie】 ==> 可说可不说，如果说一定要提到ie
```

### 17.原型链

```
1. 原型可以解决什么问题
    对象共享属性和共享方法
2. 谁有原型
    函数拥有：prototype
    对象拥有：__proto__
3. 对象查找属性或者方法的顺序
    先在对象本身查找 --> 构造函数中查找 --> 对象的原型 --> 构造函数的原型中 --> 当前原型的原型中查找
4. 原型链
    4.1 是什么？：就是把原型串联起来
    4.2 原型链的最顶端是null
```

### 18. JS继承有哪些方式

#### 1.方式一：ES6

```
class Parent{
  constructor(){
    this.age = 18;
  }
}
​
class Child extends Parent{
  constructor(){
    super();
    this.name = '张三';
  }
}
let o1 = new Child();
console.log( o1,o1.name,o1.age );
```

#### 2.方式二：原型链继

```
function Parent(){
  this.age = 20;
}
function Child(){
  this.name = '张三'
}
Child.prototype = new Parent();
let o2 = new Child();
console.log( o2,o2.name,o2.age );
```

#### 3.方式三：借用构造函数继承

```
function Parent(){
  this.age = 22;
}
function Child(){
  this.name = '张三'
  Parent.call(this);
}
let o3 = new Child();
console.log( o3,o3.name,o3.age );
```

#### 4.方式四：组合式继承

```
function Parent(){
  this.age = 100;
}
function Child(){
  Parent.call(this);
  this.name = '张三'
}
Child.prototype = new Parent();
let o4 = new Child();
console.log( o4,o4.name,o4.age );
```

### 19.说一下call、apply、bind区别

```
共同点：功能一致
    可以改变this指向
语法： 函数.call()、函数.apply()、函数.bind()
  
区别：
    1. call、apply可以立即执行。bind不会立即执行，因为bind返回的是一个函数需要加入()执行。
    2. 参数不同：apply第二个参数是数组。call和bind有多个参数需要挨个写。
  
场景：
    1. 用apply的情况
      var arr1 = [1,2,4,5,7,3,321];
      console.log( Math.max.apply(null,arr1) )
​
    2. 用bind的情况
      var btn = document.getElementById('btn');
      var h1s = document.getElementById('h1s');
      btn.onclick = function(){
        console.log( this.id );
      }.bind(h1s)
```

### 20.sort背后原理是什么？

```
V8 引擎 sort 函数只给出了两种排序 InsertionSort 和 QuickSort，数量小于10的数组使用 InsertionSort，比10大的数组则使用 QuickSort。
​
之前的版本是：插入排序和快排，现在是冒泡
```

### 21.深拷贝和浅拷贝

```
共同点：复制
​
1. 浅拷贝：只复制引用，而未复制真正的值，拷贝的就是内存地址
var arr1 = ['a','b','c','d'];
var arr2 = arr1;
​
var obj1 = {a:1,b:2}
var obj2 = Object.assign(obj1);
​
2. 深拷贝：是复制真正的值 ，从堆内存中开辟一个新的区域存放新对象,（不同引用）
var obj3 = {
  a:1,
  b:2
}
var obj4 = JSON.parse(JSON.stringify( obj3 ));//但不能处理函数和正则
​
//递归的形式
function copyObj( obj ){
  if(  Array.isArray(obj)  ){
    var newObj = [];
  }else{
    var newObj = {};
  }
  for( var key in obj ){
    if( typeof obj[key] == 'object' ){
      newObj[key] = copyObj(obj[key]);
    }else{
      newObj[key] = obj[key];
    }
  }
  return newObj;
}
console.log(  copyObj(obj5)  );
```

### 22.下面的输出结果是?

```
(function (){ 
  var a = b = 10; 
})()  
(console.log(typeof a,b)  //undefined。10
```

### 23.转换为一维数组

```
var data = [8, [3,[ [7,4],5],6]];
data.flat(Infinity)
```

### 24.打印出 age 的值

```
let obj ='{"age":"18","name":"jack"}'
```

### 25.两个没有刻度的水桶，容量分别为4升和9升，如何盛出6升水?

```
首先倒满两个四升，分别倒入九升水桶里面，再倒满四升水，然后往九升水桶里面倒入一升，还有三升，把九升水桶里面的水倒掉，把四升水桶里面剩下的三升水倒入九升水桶里面，再往四升水桶里面倒满水，然后，把水倒入九升水桶里面，现在九升水桶里面有7升水，再把四升水桶倒满，再往九升水桶里面倒入两升水，四升水桶还有两升水，九升水桶这个时候已经满了，倒掉，把四升水桶剩下的两升水倒入九升水桶中，再把四升水桶倒满，然后直接倒入九升水桶里面，这个时候九升水桶里面已经有了六升水
```

### 26.find和filter的区别

```
区别一：返回的内容不同
  filter 返回是新数组
  find   返回具体的内容
区别二：
  find ：匹配到第一个即返回
  filter ： 返回整体（没一个匹配到的都返回）
​
```

### 27.some和every的区别

```
ome  ==》 如果有一项匹配则返回true
every ==》 全部匹配才会返回true、
```

### 28.Math

```
Math.ceil()上取整
Math.round() 四舍五入
Math.floor()下取整
Math.max()最大值
Math.min()最小值
```

### 29.JavaScript中的错误类型

```
1.Error
  Error是最基本的错误类型，其他的错误类型都继承自该类型。因此，所有错误的类型共享了一组相同的属性。 这个类型的错误很少见。一般使用开发人员自定义抛出的错误。
​
2.EvalError
  这个错误会在使用eval()函数发生异常时候抛出。两种情况会出错
​
3.RangeError
  这个错误会在数值超出相应范围时触发。比如使用new Array()的时候传递一个负数或者是超过数组最大长度（4,294,967,295）的数，比如Number.MAX_VALUE，Number.MIN_VALUE。注意递归爆炸也有这个错误。
​
4.ReferenceError
  这个错误一般就是出现在变量找不到的情况，比如：
    var a = b;
    Uncaught ReferenceError ：b is not defined
    这时候就需要检查一下一个变量了
​
5.SyntaxError
  当Javascript语言解析代码时,Javascript引擎发现了不符合语法规范的tokens或token顺序时抛出SyntaxError。
​
6.TypeError
  这个错误在JavaScript中是经常遇到的，不管是初学者还是老手。在变量中保存着以外的类型时，或者在访问不存在的方法时。都会导致这种错误。但是归根结底还是由于在执行特定于类型的操作时，变量的类型并不符合要求所致。比如：
  var o = new 10;
  a.style.widht = "10px";
  关于设置样式这个东西，新手会遇到很多，一般这都是由获取不到元素导致的。
​
7.URIError
  在使用encodeURI或者decodeURI因为URL格式不正确时，就会导致URIError错误。这种错误也很少见。
```

### 30.实现一个call函数

```
复制// 思路:将要改变this指向的方法挂到目标this上执行并返回 Function.prototype.mycall = function (context) {
  if (typeof this !== 'function') {
    throw new TypeError('not funciton')
  }
  context = context || window
  context.fn = this
  let arg = [...arguments].slice(1)
  let result = context.fn(...arg)
  delete context.fn
  return result
}
```

### 31.实现一个apply函数

复制// 思路:将要改变this指向的方法挂到目标this上执行并返回 Function.prototype.myapply = function (context) {

```
  if (typeof this !== 'function') {
    throw new TypeError('not funciton')
  }
  context = context || window
  context.fn = this
  let result
  if (arguments[1]) {
    result = context.fn(...arguments[1])
  } else {
    result = context.fn()
  }
  delete context.fn
  return result
}
```

### 32.实现一个bind函数

```
复制// 思路:类似call，但返回的是函数 Function.prototype.mybind = function (context) {
if (typeof this !== 'function') {
  throw new TypeError('Error')
}
let _this = this
let arg = [...arguments].slice(1)
return function F() {
  // 处理函数使用new的情况
  if (this instanceof F) {
    return new _this(...arg, ...arguments)
  } else {
    return _this.apply(context, arg.concat(...arguments))
  }
}
}
```
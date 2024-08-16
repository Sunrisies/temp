- # Math.abs()

```js
"use strict";

//Math.abs(x)函数返回一个数字的绝对值。
let x = -30.4;
let results = Math.abs(x);
console.log(x); // -30.4
console.log(results); // 30.4
```

- # Math.ceil()
```js
"use strict";

//Math.ceil()函数返回向上取整的最接近的一个整数
let x = 5.43
let results = Math.ceil(x)
console.log(results); // 6
```

- # Math.floor()
```js
"use strict";

//Math.floor()函数返回一个向下取整的数
let x = 30.7;
let results = Math.floor(x)
console.log(results); // 30
```

- # Math.fround()
```js
"use strict";

//Math.fround()可以将任意的数字转换为它离最近的单精度数字形式的数字(32位)。
let x = 30.7;
let results = Math.fround(x)
console.log(results); // 30.700000762939453
```

- # Math.max()
```js
"use strict";

//Math.max()函数返回作为输入参数的最大数字，如果没有参数，则返回 - Infinity。
let x = [23, 45, 12, 45, 67];
let results = Math.max(...x);
console.log(results); // 67
```

- # Math.min()
```js
"use strict";

//Math.min()函数返回作为输入参数的数字中最小的一个，如果没有参数，则返回Infinity。
let x = [23, 45, 12, 45, 67];
let results = Math.min(...x);
console.log(results); // 12
```

- # Math.pow()
```js
"use strict";

//Math.pow()函数返回的是平方
let x = 3;
let results = Math.pow(x, 2);
console.log(results); // 9
```

- # Math.random()
> 在生成随机数的时候，使用这个window.crypto.getRandomValues
```js
"use strict";

//Math.random() 函数返回一个随机数是从0～1之间
let results = Math.random();
console.log(results); // 0.7854399431246668

let array = new Uint32Array(3); //随机生成三个数
console.log(window.crypto.getRandomValues(array));
for (const num of array) {
    console.log(num); // 3764019144 724316780 1362846274
}
```


- # Math.round()
```js
"use strict";

//Math.round()函数返回一个数字舍五入后最接近的四群。
let x = 3.4;
let results = Math.round(x);
console.log(results); // 3
let x1 = 3.6;
let results1 = Math.round(x1);
console.log(results1); // 4
```

- # Math.trunc()
```js
"use strict";

//Math.trunc()办法增加数字数字部分，只保留剩下的小部分。
let x = 23.45;
let results = Math.trunc(x);
console.log(results); // 23
```
























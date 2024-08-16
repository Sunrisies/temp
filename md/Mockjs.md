# Mock

我参考的不是官网，官网域名到期了，我就参考的其他的网站

## 基本数据的生成

- 字符串

```js
string|(变量/常量)
Mock.mock({ "string|1-10": "★" })
// => { "string": "★★★★★★★★★" }
随机生成1-10个★

Mock.mock({ "string|5": "★" })
// => { "string": "★★★★★" }
生成5个★
```

- 数字
```js
number|(变量/常量)
Mock.mock({ "number|+1": 712 }) 
// => Result { "number": 610 }

Mock.mock({ "number|1-100": 100 })
// => { "number": 58 }
生成1~100之间的数字

Mock.mock({ "number|1-100.1-10": 1 })
// => { "number": 50.16857 }
生成1~100之间的小数，小数的位数是1~10

Mock.mock({ "number|123.1-10": 1 })
// => { "number": 123.422702408 }
生成123的小数，小数的位数是1~10

Mock.mock({ "number|123.3": 1 })
// => { "number": 123.376 }
生成123的小数，小数的位数是一个常数3
```

- boolean
```js
Mock.mock({ "boolean|0": true })
// => { "boolean": false }
生成的全是false

Mock.mock({ "boolean|1-2": true })
// => { "boolean": true }
随机生成true或者false
```

- 对象
```js
object|(let/const)
Mock.mock({ "object|2":
        {   "310000": "上海市", 
            "320000": "江苏省", 
            "330000": "浙江省", 
            "340000": "安徽省" }
        })
       
// => { "object": { "320000": "江苏省", "340000": "安徽省" } }
如果是let 随机生成的数量是里面的任意数量 ，如果是const 随机生成的数量是常量
```

- 数组
```js
array|(常量/变量)
Mock.mock({ "array|1": [ "AMD", "CMD", "UMD" ] })
// => { "array": "UMD" }
随机生成其中一个，这个常量不是一个的话，这个常量是几就生成几遍里面的全部数据

Mock.mock({ "array|+2": [ "AMD", "CMD", "UMD" ] })
// => { "array": "AMD" }
每次生成的数据是前面的数据加这个常量，如果生成10个数据，这个常量是2，那么应该是这样排序
// => AMD => UMD => CMD => AMD => UMD => CMD => AMD => UMD => CMD => AMD
```

- 正则(RegExp)
```js
Mock.mock({ 'regexp': /[a-z][A-Z][0-9]/ })
// => { "regexp": "wN0" }
随机生成a-zA-Z0-9之间的数据

Mock.mock({ 'regexp': /\d{5,10}/ })
// => { "regexp": "8328620263" }
随机生成常数5~10个数字

Mock.mock({ 'regexp|3': /\d{5,10}\-/ })
// => { "regexp": "65641137-417814-708381-32690516-" }
随机生成3遍常数5~10个数字

Mock.mock({ 'regexp|1-5': /\d{5,10}\-/ })
// => { "regexp": "00848-216351-983292-" }
随机生成1~5遍常数5~10个数字
```

- 数据占位符定义
> boolean
```js 
Mock.mock('@boolean')  === Mock.mock('@boolean()')这两种的效果一样
// => false
随机生成false跟true，
```

> 数字 natural | integer | float
```js
natural( min?, max? )
Mock.mock('@natural') ===  Mock.mock('@natural()') 这两种的效果一样
// => 5979995984345020
随机生成数字
Mock.mock('@natural(60, 100)')
// => 80
随机生成60~100之间的数字

integer( min?, max? )
Mock.mock('@integer') === Mock.mock('@integer()') 这两种的效果一样
// => -2562947874170488
随机生成正整数跟负整数

float( min?, max?, dmin?, dmax? )
Mock.mock('@float') === Mock.mock('@float()')这两种的效果一样
// => -7872335019571184.2
随机生成数字，包括负数，小数，整数等
Mock.mock('@float(0)')
// => 564426826787310.6
随机生成数字(小数，整数)
Mock.mock('@float(60, 100)')
// => 93.564185633
随机生成60到100之间的小数
Mock.mock('@float(60, 100, 3, 5)')
// => 75.72173
随机生成60到100直接的小数，小数的位数最少是3，最大是5
```

> string( pool?, min?, max? )
```js
Mock.mock('@string("lower", 1, 3)') 
// => "zkg"   
随机生成1到3个小写字母
Mock.mock('@string("upper", 1, 3)') 
// => "LH"
随机生成1到3个大写字母
Mock.mock('@string("number", 1, 3)') 
// => "712"
随机生成1到3个数字
Mock.mock('@string("symbol", 1, 3)') 
// => "(%[" 
随机生成1到3个symnol类型的
```

> range(start?, stop, step?)
```js
Mock.mock('@range(1, 10, 3)')
// => [1,4,7]
在1到10之间生成两数之间差是3的数据，并且组成数组
```

- 日期
> date( format? )
```js
Mock.mock('@date') === Mock.mock('@date()')
// => "1972-05-18"
随机生成年月日 格式是yyyy-MM-dd

Mock.mock('@date("yyyy-MM-dd")') 
// => "2002-08-17"
Mock.mock('@date("yy-MM-dd")') 
// => "87-09-09"
Mock.mock('@date("y-MM-dd")') 
// =>  "22-02-02"
Mock.mock('@date("y-M-d")')
// =>  "09-6-13"
随机生成指定格式的年月日
```

> time( format? )
```js
Mock.mock('@time') === Mock.mock('@time()')
// => "19:06:42"
随机生成时间 格式是HH:mm:ss

Mock.mock('@time("A HH:mm:ss")') 
//=> "PM 12:58:43"  
Mock.mock('@time("a HH:mm:ss")') 
// => "pm 16:17:50"
Mock.mock('@time("HH:mm:ss")')
//=> "00:57:49"
Mock.mock('@time("H:m:s")')
// =>  "10:19:3"
随机生成指定格式的时间
```

> datetime( format? )
```js
Mock.mock('@datetime') === Mock.mock('@datetime()')
// => "1975-01-17 18:49:25"
随机生成年月日加上时间 格式是yyyy-MM-dd A HH:mm:s

Mock.mock('@datetime("yyyy-MM-dd A HH:mm:ss")') 
// =>"2001-11-25 PM 12:35:10"  
Mock.mock('@datetime("yy-MM-dd a HH:mm:ss")') 
// => "85-08-07 pm 22:17:52"
Mock.mock('@datetime("y-MM-dd HH:mm:ss")') 
// => "96-01-30 15:32:50" 
Mock.mock('@datetime("y-M-d H:m:s")')
// => "87-4-13 1:1:19"
随机生成指定格式的年月日加上时间
```

> now( unit?, format? )
```js 
Mock.mock('@now') === Mock.mock('@now()')
// => 2022-09-04 17:47:42
生成现在的时间，格式是yyyy-MM-dd A HH:mm:s

Mock.mock('@now("yyyy-MM-dd A HH:mm:ss")') 
// => "2022-09-04 PM 17:49:53"
Mock.mock('@now("yy-MM-dd a HH:mm:ss")') 
// => "22-09-04 pm 17:50:34"
Mock.mock('@now("y-MM-dd HH:mm:ss")') 
// => "22-09-04 17:50:57"
Mock.mock('@now("y-M-d H:m:s")')
// => "22-9-4 17:51:22"
生成现在的时间,指定格式的年月日加上时间
```

- 照片
> image( size?, background?, foreground?, format?, text? )
```js
Mock.mock("@image('200x100', '#894FC4', '#FFF', 'png', '!')")
// => http://dummyimage.com/200x100/894FC4/FFF.png&text=!
生成图片大小是200x100，背景颜色是#894FC4 字体颜色#FFF 图片类型为png 图片内容是！
```
<img src="http://dummyimage.com/200x100/894FC4/FFF.png&text=!" />

- 颜色
```js
Mock.mock('@color') === Mock.mock('@color()')
// => "#f28579"
随机生成颜色，颜色格式是#******
Mock.mock('@rgb') === Mock.mock('@rgb()')
// => "rgb(163, 121, 242)"
随机生成颜色。格式是rgb(***,***,***)
Mock.mock('@rgba') === Mock.mock('@rgba()')
// => "rgba(121, 184, 242, 0.29)"
随机生成颜色。格式是rgba(***,***,***,0~1)
```

- 文本
> paragraph( min?, max? )
```js
Mock.mock('@paragraph') === Mock.mock('@paragraph()')
随机生成英文的文字，行数不确定
Mock.mock('@paragraph(2)')
随机生成英文的文字，行数是2
Mock.mock('@paragraph(1, 3)')
随机生成英文的文字，行数是1~3
```

> sentence( min?, max? )
```js
Mock.mock('@sentence') === Mock.mock('@sentence()')
随机生成英文句子,一般不会超过一行
Mock.mock('@sentence(4)')
随机生成英文句子，单词一共有四个
Mock.mock('@sentence(3, 5)')
随机生成英文句子，单词一共有四个
```

> word( min?, max? )
```js
Mock.mock('@word') === Mock.mock('@word()') 
// => "wxyagtrqeg"
随机生成一个英文单词，长度不确定
Mock.mock('@word(5)') 
随机生成一个英文单词，长度是5
Mock.mock('@word(3, 5)')
随机生成一个英文单词，长度是3到5个之间
```

> title( min?, max? )
```js
Mock.mock('@title') === Mock.mock('@title()') 
随机生成英文句子，一般单词个数不是很多
Mock.mock('@title(5)')
随机生成英文句子，单词个数是5
Mock.mock('@title(3, 5)')
随机生成英文句子，单词个数是3到5个之间
```

> cparagraph( min?, max? )
```js
Mock.mock('@cparagraph') === Mock.mock('@cparagraph()') 
随机生成中文的句子,一般是在一到两行之间
Mock.mock('@cparagraph(2)') 
随机生成中文句子，生成两个句子
Mock.mock('@cparagraph(1, 3)')
随机生成中文句子，生成1到3个句子之间
```

> csentence( min?, max? )
```js
Mock.mock('@csentence') === Mock.mock('@csentence()') 
// => "除写改级争能备进们且件积写。"
随机生成一个句子，
Mock.mock('@csentence(5)')
// => "角红关年转。"
随机生成一个句子，句子中个数是5
Mock.mock('@csentence(3, 5)')
// => "小百生从。"
随机生成一个句子，句子的个数是3到5个
```
- 名字
> first()
```js
Mock.mock('@first') Mock.mock('@first()')
```

> last()
```js
Mock.mock('@last') === Mock.mock('@last()')
随机生成英文单词，是名字的第一个
```

> name( middle? )
```js
Mock.mock('@name') === Mock.mock('@name()')
随机生成英文单词，是名字的最后一个
```

>中文名字cfirst()
```js
Mock.mock('@cfirst') === Mock.mock('@cfirst()')
随机生成中文，是名字的第一个
```

> clast()
```js
Mock.mock('@clast') === Mock.mock('@clast()')
随机生成中文，是名字的最后一个
```

> cname()
```js
Mock.mock('@cname') === Mock.mock('@cname()')
随机生成中文，是中文名字,一般是两个字的名字
```

- Web
> email()
```js
生成字符串 
Mock.mock('@email') === Mock.mock('@email()') 
// => "y.lee@lewis.org"
生成对象 
Mock.mock( { email: '@email' } ) 
// => { email: "v.lewis@hall.gov" }
```

> ip()
```js
Mock.mock('@ip') Mock.mock('@ip()')
随机生成ip地址
```

- 地址
> region()
```js
Mock.mock('@region') === Mock.mock('@region()')
// => "西南"
随机生成地址，按照大区划分
```

> province()
```js
Mock.mock('@province') === Mock.mock('@province()')
随机生成省份
```

> city( prefix? )
```js
Mock.mock('@city') === Mock.mock('@city()')
// => "重庆市"
随机生成市
Mock.mock('@city(true)')
// =>"西藏自治区 阿里地区"
随机生成市，也包括上面的省
```

> county( prefix? )
```js
Mock.mock('@county') === Mock.mock('@county()')
// => "达日县"
随机生成县
Mock.mock('@county(true)')
随机生成县，也包括上面的市跟省
```

> zip()

```js
Mock.mock('@zip') === Mock.mock('@zip()')
// => "601562"
随时生成一个邮政编码
```

- Miscellaneous
> guid()
```js
Mock.mock('@guid') === Mock.mock('@guid()')
// => "590B5ebb-c345-EC28-c5De-7782e5c2F74F"
随机生成id
```

> id()
```js
Mock.mock('@id') === Mock.mock('@id()')
// => "450000199007143421"
随机生成id
```
有一些，自己使用过，有一些，自己没有使用过，如果有什么问题，可以提出来






### 1.块级元素跟行内元素区别：

```
区别：
    1.块级元素独占一行，行内元素不行
    2.块级元素可以设置宽高，行内元素不行 
    3.块级元素可以包含其他行内元素跟块级元素，行内元素只能包含文本跟其他行内元素
```

### 2.行内元素有哪些？块级元素有哪些？ 空(void)元素有哪些？

```
行内元素：span、img、input...
块级元素：div、footer、header、section、p、h1...h6...
空元素：br、hr...
元素之间的转换问题：
display: inline;       把某元素转换成了行内元素      ==>不独占一行的，并且不能设置宽高
display: inline-block;    把某元素转换成了行内块元素   ==>不独占一行的，可以设置宽高
display: block;     把某元素转换成了块元素       ==>独占一行，并且可以设置宽高
```

### 3.css3新特性

#### 1.新增各种css选择器

```
通配（*）
id选择器（#）
类选择器（.）
标签选择器（div、p、h1...）
相邻选择器(+)
后代选择器(ul li)
子元素选择器（ > ）
属性选择器(a[href])
```

#### 2.圆角border-radius

```
值：
  length 定义弯道的形状。
    四个值: 第一个值为左上角，第二个值为右上角，第三个值为右下角，第四个值为左下角。
    三个值: 第一个值为左上角, 第二个值为右上角和左下角，第三个值为右下角
    两个值: 第一个值为左上角与右下角，第二个值为右上角与左下角
    一个值： 四个圆角值相同
​
  % 使用%定义角落的形状。
```

#### 3.多列布局

```
column-count                      属性指定了需要分割的列数。
column-gap                        属性指定了列与列间的间隙。
column-rule-style                 属性指定了列与列间的边框样式：
column-rule-width                 属性指定了两列的边框厚度:
column-rule-color                 属性指定了两列的边框颜色：
column-rule                       属性是 column-rule-* 所有属性的简写
column-span                       属性是指定元素要跨越多少列
column-width                      属性是指定列的宽度
```

#### 4.阴影和反射

```
阴影
    文本阴影:text-shadow
      默认值none，表示没有阴影。
      第一个长度值用于设置文本阴影的水平偏移值，可以为负值。
      第二个长度值用于设置文本阴影的垂直偏移值，可以为负值。
      第三个长度值用于设置阴影的模糊半径，不可为负值。此参数可省略。
      颜色值用于标识阴影的具体颜色。
      
    容器阴影:box-shadow
      默认值none，表示没有阴影。
      第1个长度值用来设置对象的阴影水平偏移值。可以为负值。
      第2个长度值用来设置对象的阴影垂直偏移值。可以为负值。
      如果提供了第3个长度值则用来设置对象的阴影模糊值。不允许负值。
      如果提供了第4个长度值则用来设置对象的阴影外延值。
      inset用于设置对象的阴影类型为内阴影。该值为空时，则对象的阴影类型为外阴影 。
      
     多重阴影:文本阴影和容器阴影都支持多重阴影，就是可以给text-shadow或者box-shadow属性设置多组阴影值
​
反射
    box-reflect：<direction>  <offset>  <mask-box-image>
      none，为默认值，表示无反射。
      <direction>，反射的方向，可选值为above，below，left，right。
      <offset>，表示本体和反射之间的间隔，可设置为长度值也可设置为百分比。此参数可省略，默认为0。
      <mask-box-image>，用于设置反射的遮罩，可设置为渐变或者一个图片。此参数可省略。
      有一点需要注意一下，就是我们要为反射的方向（即<direction>参数的方向）预留足够的空间，否则会导致页面看起来无反应，因为没有足够的空间用来展示反射。
```

#### 5.渐变

```
线性渐变（Linear Gradients）
  默认情况下，linear-gradient线性渐变是从上到下开始过渡的。
​
  当然渐变的方向也可以是，向下/向上/向左/向右/对角方向，以及定义一个角度（angle）。
​
径向渐变（Radial Gradients）
  径向渐变由它的中心定义。
```

#### 7.旋转transform

```
transform不会触发回流，和重绘。
​
1.rotate(xx deg)(2D), rotateX()(3D), rotateY()(3D)：以中心为基点，deg表示旋转的角度，为负数时表示逆时针旋转。
2.translate(x,y) ，translateX(x) ，translateY(y)：以中心为基点按照设定的x,y参数值,对元素进行进行平移。
3.scale(x,y)，scaleX(x,1)， scaleY(1,Y)：缩放基数为1，如果其值大于1元素就放大，反之其值小于1为缩小。
4.skew(x,y)， skewX(x)， skewY(y)：以中心为基点，第一个参数是水平方向扭曲角度，第二个参数是垂直方向扭曲角度。
​
5，所有操作的默认的基点都在中心位置，我们可以使用transform-origin：(x,y)来改变元素基点。x可以取left，center ，right，是水平方向取值，也可以取对应的百分值为left=0%;center=50%;right=100%y可以取top ，center， bottom，是垂直方向的取值，其中top=0%;center=50%;bottom=100%;如果只取一个值，表示垂直方向值不变。
```

#### 8.伪类选择器

```
p:first-of-type           选择属于其父元素的首个< p>元素的每个< p> 元素。
p:last-of-type            选择属于其父元素的最后 < p> 元素的每个< p> 元素。
p:only-of-type            选择属于其父元素唯一的 < p>元素的每个 < p> 元素。
p:only-child              选择属于其父元素的唯一子元素的每个 < p> 元素。
p:nth-child(2)            选择属于其父元素的第二个子元素的每个 < p> 元素。
:after                    在元素之前添加内容,也可以用来做清除浮动。
:before                   在元素之后添加内容。
:enabled                  已启用的表单元素。
:disabled                 已禁用的表单元素。
:checked                  单选框或复选框被选中。
```

### 6.position的值

```
static      [默认]  没有定位
fixed       固定定位，相对于浏览器窗口进行定位。
relative      相对于自身定位，不脱离文档流。
absolute    相对于第一个有relative的父元素，脱离文档流。
​
relative和absolute区别
  1. relative不脱离文档流 、absolute脱离文档流
  2. relative相对于自身 、 absolute相对于第一个有relative的父元素
  3. relative如果有left、right、top、bottom ==》left、top
     absolute如果有left、right、top、bottom ==》left、right、top、bottom
```

### 7.css优先级算法计算

```
1.优先级就近原则，同权重下样式，定义最近者为准，
2.载入样式以最后载入的定位为准，后载入的高于前面，前者会被覆盖
3.优先级为 : !important(规则最重要，大于其他规则)>id(1000)>class(10)>tag(i);!important比内联优先级高
```

### 8.css3动画

```
过渡动画：transition
  含意：过渡就是使瞬间的样式变化，按照⼀定⽅式变得缓慢平缓
  
  transition-property 执⾏过渡的属性，例⼦设置为颜⾊color的变化，也可以是width, font-size等，不设置的话默认是all，即所有属性；
  transition-duration transition效果需要指定多少秒或毫秒才能完成
  transition-timing-function  指定transition效果的转速曲线
  transition-delay  延迟时间，即多少秒后执⾏过渡效果
​
动画：animation/keyframes
  animation-name  指定要绑定到选择器的关键帧的名称
  animation-duration  动画指定需要多少秒或毫秒完成
  animation-timing-function 设置动画将如何完成一个周期
  animation-delay 设置动画在启动前的延迟间隔。
  animation-iteration-count 定义动画的播放次数。
  animation-direction 指定是否应该轮流反向播放动画。
  animation-fill-mode 规定当动画不播放时（当动画完成时，或当动画有一个延迟未开始播放时），要应用到元素的样式。
  animation-play-state  指定动画是否正在运行或已暂停。
 
区别：
transition需要触发⼀个事件才会随着时间改变其CSS属性
animation在不需要触发任何事件的情况下，也可以显式的随时间变化来改变元素CSS属性，达到⼀种动画的效果
1、动画不需要事件触发，过渡需要
2、过渡只有⼀组（两个：开始-结束）关键帧，动画可以设置多个
```

### 9.stylus/sass/less区别

```
共同点：变量、混合、嵌套、继承、颜色混合
​
不同点
    1.less和sass语法较为严谨，less要求一定要使用大括号，sass和stylus可以通过缩进表示层级跟嵌套
    2.sass无全局变量，less和stylus有类似其他语言的作用域概念
    3.sass是基于Ruby语言的，less和styuls可以基于NodeJS NPM下载相应库进行编译
```

### 10.rgba和opacity区别

```
共同性：实现透明效果
    1. opacity 取值范围0到1之间，0表示完全透明，1表示不透明
    2. rgba   R表示红色，G表示绿色，B表示蓝色，取值可以在正整数或者百分数。A表示透明度取值0到1之间
  
  区别：继承的区别
    opacity会继承父元素的opacity属性，
    而RGBA设置的元素的后代元素不会继承不透明属性。
```

### 11.px/em/rem区别

```
px:固定值
em:随着父元素的字体大小
rem:是相对于根元素字体的大小
```

### 12.布局

#### 1.左边固定，右边自适应

##### 第一种：浮动

```
.box {
    overflow: hidden;
    border: 1px solid #000;
}
.left {
    float: left;
    width: 200px;
    height: 200px;
    background-color: green;
}
.right {
    margin-left: 200px;
    height: 150px;
    background-color:yellow;
}
    <div class="box">
        <div class="left">左边固定</div>
        <div class="right">右边自适应</div>
    </div>
```

##### 第二种父相子绝，外加margin-left

```
.box {
      position: relative;
      height: 200px;
      border: 1px solid #000;
    }
    .left {
      position: absolute;
      left: 0;
      top: 0;
      width: 200px;
      height: 100%;
      background-color: green;
    }
    .right {
      margin-left: 200px;
      height: 150px;
      background-color:yellow;
    }        
 
    <div class="box">
        <div class="left">左边固定</div>
        <div class="right">右边自适应</div>
    </div>
```

##### 第三种双float + calc()子类都开启左浮动，右边的宽度减左边的宽度

```
.box {
      overflow: hidden;
      border: 1px solid #000;
    }
    .left {
      float: left;
      width: 200px;
      height:200px;
      background-color: green;
    }
    .right {
      float: left;
      width: calc(100% - 200px);
      height: 150px;
      background-color:yellow;
    }
​
<div class="box">
  <div class="left">左边固定</div>
  <div class="right">右边自适应</div>
</div>
```

##### 第四种：flex布局，父元素开启flex，左边固定，右边flex：1

```
.box {
      display: flex;
      border: 1px solid #000;
    }
    .left {
      flex: 0 0 200px;
      height: 200px;
      background-color: green;
    }
    .right {
      flex: 1;
      height: 150px;
      background-color:yellow;
    }
```

#### 2.垂直居中

```
方式一：
<div class='container'>
  <div class='main'>main</div>
</div>
.container{
  display: flex;              伸缩盒
  justify-content: center;    水平方向
  align-items: center;        垂直方向
  width: 300px;
  height: 300px;
  border:5px solid #ccc;
}
.main{
  background: red;
}
​
方式二：
<div class='container'>
  <div class='main'>main</div>
</div>
​
.container{
  position: relative;
  width: 300px;
    height: 300px;
  border:5px solid #ccc;
}
.main{
  position: absolute;
  left:50%;
  top:50%;
  background: red;
  transform: translate(-50%,-50%);
}
```

### 13.如何实现小于12px的字体效果

```
transform：scale（0.7）
```

### 14.BFC块级格式上下文

```
BFC就是页面上一个隔离的独立容器，容器里面的子元素不会影响到外面的元素。
1. 了解BFC ： 块级格式化上下文。
2. BFC的原则：如果一个元素具有BFC，那么内部元素再怎么弄，都不会影响到外面的元素。
3. 如何触发BFC：
  float的值非none
  overflow的值非visible
  display的值为：inline-block、table-cell...
  position的值为:absoute、fixed
```

### 15.盒子模型

```
标准的是：width等于content，IE盒子width包括contengt，padding，border,IE盒子有叫怪异盒子
```

### 16.可以继承的

```
1.字体系列属性:font、font-family、font-weight、font-size、font-style;
2.文本系列属性:
  2.1）内联元素：color、line-height、word-spacing、letter-spacing、text-transform;
  2.2）块级元素：text-indent、text-align;
3.元素可见性：visibility
4.表格布局属性：caption-side、border-collapse、border-spacing、empty-cells、table-layout;
5.列表布局属性：list-style
```

### 17.不可以继承的

```
1.display：规定元素应该生成的框的类型；
2.文本属性：vertical-align、text-decoration;
3.盒子模型的属性：width、height、margin 、border、padding;
4.背景属性：background、background-color、background-image;
5.定位属性：float、clear、position、top、right、bottom、left、min-width、min-height、max-width、max-height、overflow、clip;
```

### 18.三角形

```
用边框画（border）,例如：
{
  width: 0;
  height: 0;
​
  border-left:100px solid transparent;
  border-right:100px solid transparent;
  border-top:100px solid transparent;
  border-bottom:100px solid #ccc;
}
箭头向下，上面的边框可见，其他透明，
箭头向上，下面的边框可见，其他透明
箭头向左，右边的边框可见，其他透明
箭头向右，左边的边框可见，其他透明
```

### 19.display

```
  none  元素不会显示        一般跟block配合使用
  block 此元素将显示为块级元素，此元素前后会带有换行符。
  inline  默认。此元素会被显示为内联元素，元素前后没有换行符。
  inline-block  行内块元素。（CSS2.1 新增的值）[IE6/7不支持]
  inline-table  此元素会作为内联表格来显示（类似 table），表格前后没有换行符。
  table 此元素会作为块级表格来显示（类似 table），表格前后带有换行符。
  inherit 规定应该从父元素继承 display 属性的值。
  grid 网格布局(Grid)是最强大的CSS布局方案。它将网页划分成一个个网格,可以任意组合不同的网格,做出各种各样的布局。
  flex  弹性布局，用来为盒状模型提供最大的灵活性
```

### 20.清除浮动

```
1. 触发BFC
2. 多创建一个盒子，添加样式：clear: both;
3. after方式
  ul:after{
    content: '';
    display: block;
    clear: both;
  }
```

### 21.line-height

```
line-height是每一行文字的高，如果文字换行则整个盒子高度会增大（行数*行高）。
height是一个死值，就是这个盒子的高度。
```

### 22.单行文本溢出隐藏

```
text-overflow：规定当文本溢出时，显示省略符号来代表被修剪的文本
​
white-space：设置文字在一行显示，不能换行
​
overflow：文字长度超出限定宽度，则隐藏超出的内容
```

### 23.style标签写在body前还是后

```
写在前，浏览器加载自上而下，如果先加载HTML文档没有样式，会出现页面闪烁问题
```

### 24.css sprite是什么,有什么优缺点

```
1. 是什么:把多个小图标合并成一张大图片。   又被称为精灵图
2. 优缺点
    优点：减少了http请求的次数，提升了性能。
    缺点：维护比较差（例如图片位置进行修改或者内容宽高修改）
```

### 25.css伪类跟伪元素区别

```
伪类： :focus、:hover、:active
​
伪元素：:before、:after
​
伪类本质上是为了弥补常规CSS选择器的不足，以便获取到更多信息；
​
伪元素本质上是创建了一个有内容的虚拟容器；
​
CSS3中伪类和伪元素的语法不同；
​
可以同时使用多个伪类，而只能同时使用一个伪元素
```

### 26.前端页面的三部分

```
结构层 HTML 表示层 CSS 行为层 JS
```

### 27.display: none;与visibility: hidden;的区别

```
1. 占用位置的区别
    display: none;    是不占用位置的
    visibility: hidden;     虽然隐藏了，但是占用位置
2. 重绘和回流的问题
    visibility: hidden; 、 display: none;    产生重绘
    display: none;          还会产生一次回流
    产生回流一定会造成重绘，但是重绘不一定会造成回流。
    产生回流的情况：改变元素的位置(left、top...)、显示隐藏元素....,改变物理位置
    产生重绘的情况：样式改变、换皮肤，没有改变物理位置
```

### 28.css如何实现透明度

```
共同性：实现透明效果
    1. opacity 取值范围0到1之间，0表示完全透明，1表示不透明
    2. rgba  R表示红色，G表示绿色，B表示蓝色，取值可以在正整数或者百分数。A表示透明度取值0到1之间
区别：继承的区别
    opacity会继承父元素的opacity属性，
    而RGBA设置的元素的后代元素不会继承不透明属性。
```

### 29.设置一条0.5款的线

```
/* 第一种 */
.demo1 {
    border-bottom: 1px solid #000;
    transform: scaleY(0.5);
}
​
/* 第二种 */
.demo2 {
    height: 1px;
    background: none;
    box-shadow: 0 0.5px 0 #000;
}
```

### 30.flex属性

```
Flex-direction：   弹性容器中子元素排列方式（主轴排列方式） 
    row       默认值水平方向排序
    row-reverse     水平方向倒序
    column      垂直方向排序
    column-reverse    垂直方向倒序
  
  Flex-wrap：      设置弹性盒子的子元素超出父容器时是否换行 
    nowrap      不换行
    wrap        换行
    wrap-reverse      换行时倒序
​
​
  Flex-flow：是 flex-direction 和 flex-wrap 简写形式 
    第一个值是flex-direction    第二个值是flex-wrap
​
  Justify-content：  设置弹性盒子元素在主轴上的对齐方式
    flex-start    默认值:左对齐
    flex-end    右对齐
    center      居中
    space-between 两端对齐，项目之间的间隔都相等
    space-aroun   每个项目两侧的间隔相等，所以，项目之间的间隔比项目与边框的间隔大一倍
​
  Align-item： 设置弹性盒子元素在侧轴上的对齐方式 
    flex-start  交叉轴的起点对齐
    flex-end  交叉轴的终点对齐
    center    交叉轴中点对齐
    stretch   默认值:如果项目未设置高度或设为auto，将占满整个容器的高度。
    baseline  项目的第一行文字的基线对齐
​
  Align-content：  设置行对齐  属性定义了多根轴线的对齐方式。如果项目只有一根轴线，该属性不起作用。
    flex-start    与交叉轴的起点对齐。
    flex-end    与交叉轴的终点对齐。
    center      与交叉轴的中点对齐。
    space-between 与交叉轴两端对齐，轴线之间的间隔平均分布。
    space-around  每根轴线两侧的间隔都相等。所以，轴线之间的间隔比轴线与边框的间隔大一倍。
    stretch     默认值:轴线占满整个交叉轴。
​
  项目上面的属性
  order     数值越小，排序越靠前，默认0
  flex-grow   放大比例，默认0
  flex-shrink   缩小比例，默认0
  flex-basis    定义了在分配多余空间之前，项目占据的主轴空间（main size）默认auto
  felx      是上面三个的简写，默认值0 1 auto 建议使用这个
  align-self      里面的属性跟align-items一样，是调整单个项目的位置
​
```

### 31.grid属性

grid有两种：grid块级元素；inline-grid行内元素； 容器子元素（项目）的`float`、`display: inline-block`、`display: table-cell`、`vertical-align`和`column-*`等设置都将失效。

```
gird-template-columns   每行有多少列
  grid-template-rows      有多少行
  grid-row-gap        每行间隔多少
  grid-column-gap     每列间隔多少
  grid-gap        上面两种的简写
  grid-template-areas     网格区域的名称
​
  grid-auto-flow      排序方向
    row       水平方向
    column      垂直方向
    row dense       先行后列
    column dense      先列后行
​
  justify-items       水平方向的位置
    start       对齐单元格的起始边缘。
    end       对齐单元格的结束边缘。
    center        单元格内部居中。
    stretch       拉伸，占满单元格的整个宽度（默认值）。
​
  align-items       垂直方向的位置
    start       对齐单元格的起始边缘。
    end       对齐单元格的结束边缘。
    center        单元格内部居中。
    stretch       拉伸，占满单元格的整个宽度（默认值）。
​
  place-items     是上面两种的简写形式<align-items> <justify-items>
​
  justify-content       
    start     对齐容器的起始边框
    end     对齐容器的结束边框。
    center      容器内部居中
    stretch     项目大小没有指定时，拉伸占据整个网格容器。
    space-around  每个项目两侧的间隔相等。所以，项目之间的间隔比项目与容器边框的间隔大一倍
    space-between 项目与项目的间隔相等，项目与容器边框之间没有间隔。
    space-evenly    项目与项目的间隔相等，项目与容器边框之间也是同样长度的间隔
​
  align-content
    start     对齐容器的起始边框
    end     对齐容器的结束边框。
    center      容器内部居中
    stretch     项目大小没有指定时，拉伸占据整个网格容器。
    space-around  每个项目两侧的间隔相等。所以，项目之间的间隔比项目与容器边框的间隔大一倍
    space-between 项目与项目的间隔相等，项目与容器边框之间没有间隔。
    space-evenly    项目与项目的间隔相等，项目与容器边框之间也是同样长度的间隔
​
  palce-content 是上面两种的简写形式<align-content> <justify-content>
​
  fr      表示按比例
  minmax    (最小值，最大值)长度范围
  auto      由浏览器决定长度
  repeat()    如果遇到重复的可以使用这个简化 repeat(3,33.33%) 3列
​
```

\
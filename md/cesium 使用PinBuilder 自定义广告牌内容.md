cesium提供的广告牌



![image.png](http://blog.chaoyang1024.top:9000/images/19c01ba8-065b-4429-b824-ca236f285404.png)


如果没有形状要求，可以使用cesium官方的广告牌，只需要修改内容跟颜色就可以了
```js
const pinBuilder = new Cesium.PinBuilder();
viewer.entities.add({
      position: cartesian,
      billboard: {
        image:  pinBuilder.fromText(index, color, 38).toDataURL(),
        pixelOffset: new Cesium.Cartesian2(0, -20)
      }
    })
```



![image.png](http://blog.chaoyang1024.top:9000/images/9fb9a5c7-559a-41d3-b40e-c81df3c2148f.png)
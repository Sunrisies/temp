环境

-   node 16.15.0
-   yarn 1.22.19
-   Pm2 5.2.2

步骤

-   拿到项目之后，是这样

    ![image-20230426190949458.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/431b4672e262464d8db3f664a351a381~tplv-k3u1fbpfcp-zoom-1.image)

<!---->

-   使用命令行 yarn dev 会生成一个.nuxt文件，

![image-20230426191112183.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/1feb4e17bde446649e29a4557e088774~tplv-k3u1fbpfcp-zoom-1.image)

-   接下来使用命令 yarn build,现在查看文件夹.nuxt 下面会生成一个dist文件夹

![image-20230426191321078.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/cc7a478b872c4a20ba3cd4ba784f3ea3~tplv-k3u1fbpfcp-zoom-1.image)

-   把下面文件上传到服务器上面

![image-20230426191737676.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/c19c782b70874219bb4a23c512c6722e~tplv-k3u1fbpfcp-zoom-1.image)

-   上传完服务器是这样的

![image-20230426192116504.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/4a35f18e847c49ff9d2b928c2b0a4ffe~tplv-k3u1fbpfcp-zoom-1.image)

-   先使用 yarn 命令安装依赖

    -   然后使用yarn pm2来启动服务，没有pm2 使用yarn add -g pm2 或者 yarn global add pm2
    -   启动成功是这样

![image-20230426192735658.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/3e2992b84a6f4c23b45b1e2622a1f8d5~tplv-k3u1fbpfcp-zoom-1.image)

-   配置nginx

```
server {
    listen 80;
    server_name chaoyangqq.top;#自己的域名或者ip地址
​
    location / {
        proxy_pass http://127.0.0.1:9999; #当前服务起的端口
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
```

现在就可以通过服务器去项目了，通过检查源码可以找到数据，login是写死的数据，cardsearch是调用接口获取的数据
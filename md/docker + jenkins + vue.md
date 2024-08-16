# 第一步安装docker
- 方案一： 通过宝塔控制面板去安装docker
![image.png](http://blog.chaoyang1024.top:9000/images/f62d0eec-a733-4e40-9a74-5576ba03cdce.png)
![image.png](http://blog.chaoyang1024.top:9000/images/a8caf1eb-fdec-4dfc-adcb-1a604d434b49.png)

# 第二步安装jenkins
- 方案一：通过宝塔控制面板去安装jenkins
    - 使用命令去安装 docker pull jenkins/jenkins
    - 通过命令去启动jenkins
    ```js
    docker run > -itd -u root  -p 2000:8080  -v /var/jenkins_home:/var/jenkins_home  -v /var/run/docker.sock:/var/run/docker.sock  -v /usr/bin/docker:/usr/bin/docker  --name jenkins-master >
    ```
    ``一定要把端口放开``打开2000的端口，这个时候会使用jenkins密码
    使用命令 `docker ps`查看当前启动的容器
    使用`docker logs 加容器的名称`

    ![image.png](http://blog.chaoyang1024.top:9000/images/a4445f6d-a11e-45b3-9000-26530d656d56.png)
    
    ![image.png](http://blog.chaoyang1024.top:9000/images/b4497ab7-7a75-4883-8b88-f4953150a8b0.png)


# 第三步配置jinkins的插件

- 安装ssh跟github跟nodejs之后进行重启，直接重启docker这个服务，通过宝塔去重启

![image.png](http://blog.chaoyang1024.top:9000/images/a8cf52ea-6ffc-48ae-91ad-e316dfc92a20.png)

![image.png](http://blog.chaoyang1024.top:9000/images/23fc43b6-f35b-4708-8c44-d08aab5dc9b0.png)

在全局配置下 使用淘宝镜像网站：<https://npm.taobao.org/mirrors/node/>

![image.png](http://blog.chaoyang1024.top:9000/images/8592c924-bbfb-4d29-8f8a-d9c24ffbdb10.png)

测试node是否生效

![image.png](http://blog.chaoyang1024.top:9000/images/a51021d2-733b-4820-a62c-d3f7b9db77d5.png)

如果报错 

![image.png](http://blog.chaoyang1024.top:9000/images/4f48c56d-e5a8-42e1-bb4e-da4f2d6f7dd2.png)
在宝塔中找到 /var/jenkins_home/tools/jenkins.plugins.nodejs.tools.NodeJSInstallation/node16/下面还有一层 

![image.png](http://blog.chaoyang1024.top:9000/images/3da360d4-924f-449d-b7ef-abef87227e88.png)

![image.png](http://blog.chaoyang1024.top:9000/images/af8de54d-66ef-4466-9334-1b69bc8f69e5.png)

成功解决 
![image.png](http://blog.chaoyang1024.top:9000/images/75964d47-003f-4e95-aad7-023cb341ad35.png)
# 第四步初始化项目
使用vite去创建项目,然后在github上面新建一个仓库，上传到仓库里面
# 第五步创建流水线

![image.png](http://blog.chaoyang1024.top:9000/images/3386e403-0aa9-42c5-9a0d-8d2ff92768b8.png)

![image.png](http://blog.chaoyang1024.top:9000/images/95ad9664-6215-4a85-a71b-2587a164a53e.png)

![image.png](http://blog.chaoyang1024.top:9000/images/951fd84b-e81b-4694-b001-238a3f29f158.png)

![image.png](http://blog.chaoyang1024.top:9000/images/cbacbe11-e283-4ff0-b1f7-015b7be9c3ad.png)

![image.png](http://blog.chaoyang1024.top:9000/images/889b19ee-07da-464b-a5c9-f6136c366a5d.png)

![image.png](http://blog.chaoyang1024.top:9000/images/8fae9c3d-b767-4620-9dfe-5de8597a74a3.png)

## 报错
![image.png](http://blog.chaoyang1024.top:9000/images/496e4d8b-bab8-4e9e-89af-eefe31061439.png)
- 方案一

![image.png](http://blog.chaoyang1024.top:9000/images/26c1836c-bdeb-4dec-abd9-b9ee1973e05b.png)

- 方案二 重试

# 完成
访问`20000`端口，一定要把端口放开

![image.png](http://blog.chaoyang1024.top:9000/images/4654e3b8-2240-44df-a7ef-b52b902fcc9e.png)

本地对代码进行修改，提交代码到github成功触发

![image.png](http://blog.chaoyang1024.top:9000/images/ebada3c8-d1be-407f-b9dd-a9bdaeb485d5.png)

访问20000端口

![image.png](http://blog.chaoyang1024.top:9000/images/899c5c6a-2e9b-45ab-8191-33c89e54d901.png)

说明修改成功
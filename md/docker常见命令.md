学习：Docker 是一个开源的平台，用于开发、交付和运行应用程序。以下是一些常见的 Docker 命令，用于构建、运行和管理容器：

### 容器生命周期管理:

1.  **构建镜像:**

    ```js
    docker build -t image_name:tag .
    ```

2.  **列出本地镜像:**

    ```
    docker images
    ```

3.  **运行容器:**

    ```
    docker run -d -p 本地端口:容器的端口 镜像名称:版本
    ```

4.  **列出运行中的容器:**

    ```
    docker ps
    ```

5.  **列出所有容器（包括停止的）:**

    ```
    docker ps -a
    ```

6.  **停止容器:**

    ```
    docker stop 容器id
    ```

7.  **启动已停止的容器:**

    ```
    docker start 容器id
    ```

8.  **删除容器:**

    ```
    docker rm 容器id
    ```

9.  **删除镜像:**

    ```
    docker rmi 镜像名称:版本
    ```

### 容器执行操作:

10. **进入运行中容器:**

    ```
    docker exec -it 容器名称 /bin/bash
    ```

11. **查看容器日志:**

    ```
    docker logs 容器id
    ```

### 网络和数据卷:

12. **列出 Docker 网络:**

    ```
    docker network ls
    ```

13. **创建 Docker 网络:**

    ```
    docker network create 网络名称
    ```

14. **列出数据卷:**

    ```
    docker volume ls
    ```

15. **创建数据卷:**

    ```
    docker volume create 数据卷名称
    ```

### 其他常用命令:

16. **查看 Docker 版本:**

    ```
    docker version
    ```

17. **查看 Docker 信息:**

    ```
    docker info
    ```
18. **查看所有的容器id:**
    ```
    docker ps -a -q
    ```

19. **停止并删除所有容器:**

    ```
    docker stop $(docker ps -a -q) && docker rm $(docker ps -a -q)
    ```

20. **清理无用的镜像、容器和数据卷:**

    ```
    docker system prune -a
    ```

这只是 Docker 命令的一个简短列表，Docker 提供了更多的功能和选项。你可以使用 `docker --help` 或查阅 Docker 官方文档来获取更详细的信息。
来源是gpt
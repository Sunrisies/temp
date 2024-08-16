### 第一阶段使用docker直接对文件进行打包
Dockerfile
```Dockerfile
FROM node:18.17.0
WORKDIR /temp
COPY package.json .
RUN yarn --registry https://registry.npmmirror.com/  
COPY . .
RUN yarn build

FROM node:18.17.0
WORKDIR /app
COPY next.config.mjs ./next.config.mjs
COPY public ./public
COPY .next ./.next
COPY package.json ./package.json
RUN yarn --production --registry https://registry.npmmirror.com/  
EXPOSE 3000
CMD [ "npm", "start" ]
```
使用命令：`docker build -t next-server:v1 .`

查看打包完成的镜像大小: `docker images`

![image.png](http://blog.chaoyang1024.top:9000/images/1ce08d42-d5bf-4d0e-b4eb-56fdffe2be72.png)

### 第二阶段优化了dockerfile，优化了打包大小 (多阶段构建)以及使用alpine这个会安装一个最小的版本

```js
FROM node:lts-alpine as build-stage
WORKDIR /temp
COPY package.json .
RUN yarn --registry https://registry.npmmirror.com/  
COPY . .
RUN yarn build

FROM node:lts-alpine as production-stage
WORKDIR /app
COPY --from=build-stage /temp/next.config.mjs ./next.config.mjs
COPY --from=build-stage /temp/public ./public
COPY --from=build-stage /temp/.next ./.next
COPY --from=build-stage /temp/package.json ./package.json

RUN yarn --production --registry https://registry.npmmirror.com/  
EXPOSE 3000
CMD [ "npm", "start" ] 
```

使用命令：`docker build -t next-server:v1 .`

查看打包完成的镜像大小: `docker images`

![image.png](http://blog.chaoyang1024.top:9000/images/f1674771-f5c3-4ed2-a979-8b726d4e8247.png)
### 使用next.config.mjs 进行配置，优化了一下打包大小，配置next.config.mjs
__如果使用output就不能使用`pnpm`__

```js
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
};
export default nextConfig;
```

```js
FROM node:lts-alpine as build-stage
WORKDIR /temp
COPY package.json .
RUN yarn --registry https://registry.npmmirror.com/  
COPY . .
RUN yarn  build

FROM node:lts-alpine as production-stage
WORKDIR /app
COPY --from=build-stage /temp/next.config.mjs ./next.config.mjs
COPY --from=build-stage /temp/public ./public
COPY --from=build-stage /temp/.next/standalone  ./standalone
COPY --from=build-stage /temp/package.json ./package.json
CMD [ "node", "./standalone/server" ]
```

使用命令：`docker build -t next-server:v1 .`

查看打包完成的镜像大小: `docker images`

![image.png](http://blog.chaoyang1024.top:9000/images/b77125f0-3ffa-42b0-8954-024432a3df7a.png)
`由最一开始的包为3G变成近200M`

使用命令跑一下镜像

`docker run -d -p 3300:3000 --name nextServer next-server:v1`

![image.png](http://blog.chaoyang1024.top:9000/images/1c237562-fc48-4c94-815c-23661099e1a2.png)

参考资料
- next官网 `https://nextjs.org/docs/app/api-reference/next-config-js/output`
- dcoker官网 `https://docs.docker.com/build/building/packaging/`

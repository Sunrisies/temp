
-  使用这种方式对nest打包，结果是
```dockerfile
FROM alpine:latest as build-stage
RUN apk add --no-cache --update nodejs npm yarn
WORKDIR /temp
COPY dist  ./dist
COPY prisma ./prisma
COPY package.json .
COPY tsconfig.json .
COPY nest-cli.json .
COPY tsconfig.build.json .
COPY .env .
RUN yarn    --registry https://registry.npmmirror.com/  
CMD ["sh", "-c", "node dist/src/main.js"]
```
![image.png](http://blog.chaoyang1024.top:9000/images/e02be803-8f3a-4429-bd43-d95a9533045a.png)

- 优化
```dockerfile
FROM alpine:latest as development
RUN apk add --no-cache --update nodejs npm yarn

WORKDIR /usr/src/app

COPY --chown=node:node package*.json ./
COPY prisma ./prisma
COPY tsconfig.json .
COPY nest-cli.json .
COPY tsconfig.build.json .
COPY .env .
RUN npm ci

COPY --chown=node:node . .

USER node

FROM node:18-alpine As build

WORKDIR /usr/src/app

COPY --chown=node:node package*.json ./

COPY --chown=node:node --from=development /usr/src/app/node_modules ./node_modules
COPY --chown=node:node --from=development /usr/src/app/prisma ./prisma
COPY --chown=node:node --from=development /usr/src/app/package*.json ./
COPY --chown=node:node --from=development /usr/src/app/tsconfig*.json ./
COPY --chown=node:node --from=development /usr/src/app/nest-cli.json ./

COPY --chown=node:node . .

RUN npm run build

ENV NODE_ENV production

RUN npm ci --only=production && npm cache clean --force

USER node


FROM alpine:latest as production
RUN apk add --no-cache --update nodejs npm yarn

COPY --chown=node:node --from=build /usr/src/app/node_modules ./node_modules
COPY --chown=node:node --from=build /usr/src/app/dist ./dist
COPY --chown=node:node --from=build /usr/src/app/package*.json ./
COPY --chown=node:node --from=build /usr/src/app/tsconfig*.json ./
COPY --chown=node:node --from=build /usr/src/app/nest-cli.json ./
COPY --chown=node:node --from=build /usr/src/app/.env .

CMD [ "node", "dist/src/main.js" ]
```

打包结果

![image.png](http://blog.chaoyang1024.top:9000/images/b99340f8-f307-4b80-b048-83b4d59615e1.png)

运行结果

![image.png](http://blog.chaoyang1024.top:9000/images/df355996-9169-4a3f-ac12-4600fdce6c47.png)

-减少了`300M`左右


参考资料`https://www.tomray.dev/nestjs-docker-production`
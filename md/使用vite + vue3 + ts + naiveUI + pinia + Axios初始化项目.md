# 使用vite

```
 yarn create vite
```

![image-20220708105046894](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/ce39b2fb88084c68a4eb9de3a7b6f224~tplv-k3u1fbpfcp-zoom-1.image)

# 2.安装路由

```
yarn add vue-router@4
```

`对vite进行配置添加了路由`

```
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
​
import { resolve } from 'path';
function pathResolve(dir: string) {
  return resolve(process.cwd(), '.', dir);
}
​
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue()
  ],
  resolve: {
    alias: [
      {
        find:'@',
        replacement:pathResolve('src')
      }
    ]
  },
})
​
```

`添加router/index.ts`

```
import {createRouter,createWebHistory} from 'vue-router'
​
const routes = [
  {
    path:'/',
    name:'index',
    component:() => import('@/views/Home.vue')
  },
  {
    path:'/about',
    name:'about',
    component:() => import('@/views/About.vue')
  }
]
​
export const router = createRouter({
  history:createWebHistory(),
  routes
})
```

`main.ts`

```
import { createApp } from 'vue'
import App from './App.vue'
import {router} from './router'
​
const app = createApp(App)
app.use(router)
app.mount('#app')
```

`修改App.vue`

```
<template>
  <router-view></router-view>
</template> 
```

# 3.安装Axios

```
yarn add axios
```

`添加utils/request.ts`

```
// 1. 引入axios依赖包
import axios from "axios";
// 2. axios创建对象
const request = axios.create({
    baseURL: "https://api.shop.eduwork.cn/", //管理后台要使用的接口的基地址
    timeout: 20000, //超时时间
})
​
// 3. ，请求拦截器，请求发送出去之前触发的
request.interceptors.request.use((config) => {
  //config 接口请求的配置信息
​
  return config;
}, (error) => {
  //报错的是定义前置拦截器时候抛出一个报错的信息
  return Promise.reject(error);
});
​
request.interceptors.response.use((response) => {
  //响应回来的数据操作
  return response.data;
​
}, (error) => {
  //报错的是时候抛出一个报错的信息
  return Promise.reject(error);
})
​
// 5. 抛出对象的信息
export default request;
```

# 4.安装ui库

```
yarn add -D naive-ui vfonts
```

`进行按需引入`

```
yarn add -D unplugin-vue-components
```

`对·vite进行配置`

```
import Components from 'unplugin-vue-components/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
​
plugins: [
    vue(),
    Components({
      resolvers: [NaiveUiResolver()]
    })
  ],
```

# 5.安装css库

`安装less`

```
yarn add less less-loader -D
```

`添加styles/index.less`

`安装Tailwind CSS`

```
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

`tailwind.config.js`

```
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

`添加styles/tailwind.css`

```
@tailwind base;
@tailwind components;
@tailwind utilities;
​
/*解决冲突ul库跟css*/
@layer base {
  button, [type='button'], [type='reset'], [type='submit'] {
      background-color: var(--n-color);
  }
}
```

`main.ts`

```
import './styles/tailwind.css'
import './styles/index.less'
```

# 5.安装pinia状态管理

```
yarn add pinia
```

`main.ts`

![image-20220708113158028](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/bd9c9a3f4d7949f9890c8c389c1f47b1~tplv-k3u1fbpfcp-zoom-1.image)

`store/user.ts`

```
import {defineStore} from 'pinia';
​
export interface IUserState {
  
}
​
export const useUserStore = defineStore({
    id: 'app-user',
    state: ():IUserState => ({
        
    }),
    getters: {
        getToken(): string {
            return this.token;
        },
    },
    actions: {
        setToken(token: string) {
​
        },
    }
})
```

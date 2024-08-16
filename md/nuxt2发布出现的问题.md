# 在 nuxt2 进行打包的时候，遇到图标不出现

解决办法

## 第一种解决方法

- 第一步：在 nuxt2 中 nuxt.conifg.jd 中使用 srcDir: 'src/',把所有文件放在 src 下面，
  `nuxt.config.js`

```js
import path from 'path'
export default {
  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: 'e',
    htmlAttrs: {
      lang: 'en'
    },
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
      { name: 'format-detection', content: 'telephone=no' }
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }]
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: ['element-ui/lib/theme-chalk/index.css'],
  srcDir: 'src/',
  alias: {
    '@': path.resolve(__dirname, './src/'),
    '~': path.resolve(__dirname, './src/')
  },
  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: ['@/plugins/element-ui'],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/eslint
    '@nuxtjs/eslint-module'
  ],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    // https://go.nuxtjs.dev/axios
    '@nuxtjs/axios'
  ],

  // Axios module configuration: https://go.nuxtjs.dev/config-axios
  axios: {
    // Workaround to avoid enforcing hard-coded localhost:3000: https://github.com/nuxt-community/axios-module/issues/308
    baseURL: '/'
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
    transpile: [/^element-ui/]
    // publicPath: '/assets/'
  }
}
```

目录结构

![image.png](http://blog.chaoyang1024.top:9000/images/a64996dd-544c-436e-be36-69278899f60e.png)

- 第二步：这个时候进行打包发布，会找不到`link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }]`,

- 第三步：在上传文件到服器的时候，需要上传以下几个文件

![image.png](http://blog.chaoyang1024.top:9000/images/81b87a7b-a35e-4c3b-86d4-9c0c76033c9f.png)
上传到服务器就是这样，static 这个文件必须放在 src 下面

![image.png](http://blog.chaoyang1024.top:9000/images/e707df85-3ac4-4019-aa01-aebcbd3bbad8.png)

- 第四步：接下来就可以直接 yarn，然后 yarn start，就可以访问到了（前提是已经配置好了 nginx 进行转发，进行转发的时候，不要忘记配置服务器端口）

![image.png](http://blog.chaoyang1024.top:9000/images/9314ac60-4d35-45b9-bb3c-3e3bf9eccc2f.png)

## 第二种解决方法

- 可以直接使用`link: [{ rel: 'icon', type: 'image/x-icon', href: '线上地址' }`

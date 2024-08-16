import inquirer from 'inquirer';
import ora from 'ora';
import fs from 'fs';
import * as Minio from 'minio';
import * as uuid from 'uuid';
import fetch from 'node-fetch';
import path from 'path';

(async () => {
  
  await inquirer
    .prompt([
      // { message: '请输入掘金cookie', name: 'cookie', type: 'password' },
      { message: 'minio服务地址', name: 'minioPath' },
      { message: 'minio端口', name: 'port', default: 9000 },
      { message: 'minio用户名', name: 'accessKey' },
      { message: 'minio密码', name: 'secretKey', type: 'password' },
      { message: 'minio bucket', name: 'bucketName', default: 'images' },
      { message: 'minio请求协议', type: 'input', name: 'protocol', default: 'http' },
      { message: '图片域名', name: 'url' }
    ])
    .then(async (answers) => {
      const minioPath = 'blog.chaoyang1024.top'
      const port = 9000
      const accessKey = 'hover'
      const secretKey = 'hover123456'
      const bucketName = 'images'
      const protocol = 'http'
      const url = 'blog.chaoyang1024.top'
      // const {  minioPath, port, accessKey, secretKey, bucketName, protocol, url } = answers;
      const cookie = '__tea_cookie_tokens_2608=%257B%2522user_unique_id%2522%253A%25227283778542965671463%2522%252C%2522web_id%2522%253A%25227283778542965671463%2522%252C%2522timestamp%2522%253A1709780827297%257D; sid_guard=0434c1bc94e6a968a5051a65e1a2dad0%7C1709781358%7C31536000%7CFri%2C+07-Mar-2025+03%3A15%3A58+GMT; uid_tt=7f0c1fb0848e2a10f183714cfb467233; uid_tt_ss=7f0c1fb0848e2a10f183714cfb467233; sid_tt=0434c1bc94e6a968a5051a65e1a2dad0; sessionid=0434c1bc94e6a968a5051a65e1a2dad0; sessionid_ss=0434c1bc94e6a968a5051a65e1a2dad0; sid_ucp_v1=1.0.0-KDcwMTlkN2Y3ZDNiYjZiODBkNjAxYTNkOGMwMmY4MjE1NmIxZDY5YTcKFwjuo_C6uoztBBDu4qSvBhiwFDgHQPQHGgJscSIgMDQzNGMxYmM5NGU2YTk2OGE1MDUxYTY1ZTFhMmRhZDA; ssid_ucp_v1=1.0.0-KDcwMTlkN2Y3ZDNiYjZiODBkNjAxYTNkOGMwMmY4MjE1NmIxZDY5YTcKFwjuo_C6uoztBBDu4qSvBhiwFDgHQPQHGgJscSIgMDQzNGMxYmM5NGU2YTk2OGE1MDUxYTY1ZTFhMmRhZDA; store-region=cn-bj; store-region-src=uid; test=21; _tea_utm_cache_576092={%22utm_source%22:%22gold_browser_extension%22}; _tea_utm_cache_2018={%22utm_source%22:%22gold_browser_extension%22}; _tea_utm_cache_2608={%22utm_source%22:%22gold_browser_extension%22}; csrf_session_id=1c5758a4426ffc69e576b6fe954b97ab'
      if (!cookie) {
        ora().fail('cookie不能为空');
        return;
      }

      if (!minioPath) {
        ora().fail('minio服务地址不能为空');
        return;
      }

      if (!accessKey) {
        ora().fail('minio用户名不能为空');
        return;
      }

      if (!secretKey) {
        ora().fail('minio密码不能为空');
        return;
      }

      if (!url) {
        ora().fail('图片域名不能为空');
        return;
      }


      let spinner = ora().start('开始获取文章列表数据');

      const data = await fetch('https://api.juejin.cn/content_api/v1/article/list_by_user?aid=2608&uuid=7056', {
        method: 'POST',
        headers: {
          'Cookie': cookie,
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        }
      }).then(res => res.json());

      spinner.succeed('获取文章列表数据成功');

      const minioClient = new Minio.Client({
        endPoint: minioPath,
        port: Number(port),
        useSSL: false,
        accessKey,
        secretKey,
      });

      for (let i = 0; i < data.data.length; i += 1) {
        const item = data.data[i];
        spinner = ora().start(`正在下载 ${item.article_info.title} ${i + 1} / ${data.data.length}`);
        await contentToFile(item.article_info.draft_id, item.article_info.title, cookie);
        spinner.succeed(`${item.article_info.title} 文章下载成功 ${i + 1} / ${data.data.length}`);
      }
      spinner.succeed(`所有文章下载成功`);

      async function contentToFile(id, title) {
        return new Promise(async (resolve) => {

          const data = await fetch('https://api.juejin.cn/content_api/v1/article_draft/detail?aid=2608&uuid=7056220463659533837', {
            method: 'POST',
            headers: {
              'Cookie': cookie,
              'Content-Type': 'application/json',
              'Accept': 'application/json',
            },
            body: JSON.stringify({
              draft_id: id
            })
          }).then(res => res.json());

          const imagePaths = [];

          const content = data.data.article_draft.mark_content;

          const newContent = content.replaceAll(/\!\[(.+?)\]\((.+)\)/g, ($1, $2, $3) => {

            if ($2 !== 'image.png' && !$2.includes('.gif')) {
              return $1;
            }

            const id = uuid.v4();

            imagePaths.push({
              key: id,
              path: $3,
            })

            return $1.replace($3, `${protocol}://${url}/${bucketName}/${id}.png`);
          });


          for (let i = 0; i < imagePaths.length; i += 1) {
            const { key, path } = imagePaths[i];
            const buffer = await downloadImg(path, cookie);
            await minioClient.putObject(bucketName, `${key}.png`, buffer);
          }

          const dirPath = path.resolve(process.cwd(), `md`);

          if (!fs.existsSync(dirPath)) {
            fs.mkdirSync(dirPath);
          }

          fs.writeFile(path.resolve(dirPath, `${title}.md`), newContent, (err) => {
            if (err) {
              console.log(err);
            } else {
              resolve();
            }
          });
        })
      }

    })
})()


function downloadImg(url, cookie) {
  return fetch(url, {
    headers: {
      Cookie: cookie,
    }
  })
    .then(res => res.arrayBuffer())
    .then(buffer => Buffer.from(buffer));
}





# 1.关机和重启

-   `shutdown -h now` 立即关机
-   `shutdown -h 5` 5分钟之后关机
-   `poweroff` 立即关机
-   `shutdown -r now` 立即重启
-   `shutdown -r 5` 5分钟之后重启
-   `reboot` 立即重启

# 2.目录操作命令

## 2.1.ls列出文件和目录

-   `ls`列出文件和目录
-   `ls -la` 显示所有文件和目录（包括隐藏）
-   `ll`跟 `ls` 效果差不多

## 2.2.cd切换工作目录

-   `cd ~` 进入自己的主目录
-   `cd ..` 返回上一层目录

## 2.3.mkdir创建目录（文件夹）

-   `mkdir 要创建的目录名称`
-   `mkdir -p 要创建的目录名称` 例如 `mkdir -p one/two/three`

## 2.4.cp复制文件或者目录

-   `cp -r 若给出的源文件是一个目录文件，此时将复制该目录下所有的子目录和文件`

## 2.5.mv移动并重命名

-   `mv 被改的文件名 改成之后的文件名` 文件改名 例如 `mv index.html index2.html`
-   `mv 要隐藏文件名 .要隐藏文件名` 隐藏文件 例如 `mv index.html .index.html`
-   `mv /home/www/index.html /home/static/` 仅仅移动
-   `mv /home/www/index.html /home/static/index2.html` 移动又重命名
-   `mv /home/www/website/* /home/www/static` 批量移动

## 2.6.rm删除一个文件或者目录

-   `rm 被删除的文件名或者目录` 例如 `rm file`
-   `rm -r *` 删除当前目录所有文件跟目录
-   `rm -rf *` 直接删除当前目录下面的所有文件跟目录，强制性的

# 3.文件操作命令

## 3.1.touch创建文件

-   `touch 要创建的文件` 例如 `touch new_file`

## 3.2.echo打印输出

-   `echo 打印的内容`
-   `echo 写入内容 > 被写入的文件` 例如 `echo "test content" > index.html`
-   `echo 追加内容 >> 被追加的文件` 例如 `echo "test content" >> index.html`

## 3.3.cat连接文件并打印输出

-   `cat 被查看的文件名称` 查看文件内容
-   `echo 追加内容 >> 被追加的文件` 例如 `echo "test content" >> index.html`

## 3.4.vi/vim

基本上 vi/vim 共分为三种模式，分别是**命令模式（Command mode）** ，**输入模式（Insert mode**）和**底线命令模式（Last line mode）**

一进去就是`命令模式`

-   `i`切换到输入模式。
-   `x` 删除当前光标所在处的字符。
-   `:` 切换到底线命令模式。

按下`i`，变成`输入模式`。输入模式下，左下角有 `-- INSERT --` 标志：

输入结束之后，按下ESC回到`命令模式`

如果要保存退出,先按下`:`,进入`底线命令模式`

-   `w`保存文件
-   `q`退出程序

输入 `wq` 表示保存退出 输入 `wq!` 表示强制保存并退出

# 4.压缩文件操作

## 4.1.压缩文件

-   Linux中打包文件：a.tar

-   Linux中压缩文件：a.gz

-   Linux中打包并压缩的文件：.tar.gz

    -   `tar -zcvf 压缩之后的名称 被压缩的文件或者目录` 例如 `tar -zcvf demo.tar *` 把当前目录下面的所有文件都压缩

## 4.2.解压文件

-   `tar -zxvf 压缩文件` 其中 `x`代表解压，表示把当前文件解压到当前目录下面
-   `tar -zxvf 解压文件 -C 指定解压的地方`

# 5.查找命令

## 5.1.find搜索目录

-   `find 目录 参数 文件名称` 例如 `find /usr/tmp -name 'a*'` 查找/usr/tmp目录下的所有以a开头的目录或文件

## 5.2.whereis定位课执行文件

## 5.3.which命令的作用是在PATH变量指定的路径中，搜索某个系统命令的位置，并且返回第一个搜索结果。根上面的作用很相似

# 6.su，sudo

## 6.1.su切换身份
- `su demo` 切换为 `demo` 的用户

## 6.2.sudo使用普通用户临时具有root权限

# 7.其他命令

## 7.1.whoami 显示当前用户名

## 7.2.pwd显示当前目录

## 7.3. ps -ef查看所有正在运行的进程

## 7.4.kill pid 或者 kill -9 pid(强制杀死进程) pid:进程号

## 7.5.****网络通信命令****

-   `ifconfig` 查看网卡信息
-   `ping ip` 查看网络情况
-   `netstat -an` 查看当前系统端口
-   `netstat -an | grep 8080` 搜索指定端口
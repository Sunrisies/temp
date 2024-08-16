- systemctl status firewalld 查看当前防火墙状态
- systemctl start | stop | restart firewalld 防火墙启动、关闭、重启
- systemctl enable firewalld 开机自启

## 两种编辑防火墙常用的安全策略设置
- 第一种
    - vim /etc/firewalld/zones/public.xml
- 第二种
    - firewall-cmd --permanent --add-port=2000/tcp 放行端口2000，其他功能不常用
    - firewall-cmd --list-ports 查看所有开放端口
    - firewall-cmd --state 查看当前状态
    - firewall-cmd --reload 重新并生效新的策略配置
    - firewall-cmd --query-port=3306/tcp 查看当前3306端口是否被放行

`如果修改完配置之后，必须要重启`
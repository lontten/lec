# lec
A library for creating powerful modern CLI applications. 一个用于创建功能强大的现代CLI应用程序的库

```shell
lec 命令主体

command 子命令 只能由 大小写字母,数字,中划线,下划线 组成

param 参数 只能由 大小写字母,数字,中划线,下划线 组成
option 选项,配置开关有长命令和短命令
短命令 只能由 大小写字母 组成
长命令 只能由 大小写字母,数字,中划线,下划线 组成

短命令不能有参数
长命令可以有参数

每个配置开关可以有 *个参数
示例：
-h --help			//简洁介绍
-H --helps		 //详细介绍，当没有详细介绍时，显示简洁介绍， 后面可以带多个参数，进行多 关键词搜索
-v --version

lec [param]*

lec [command]* [param]* //这种情况下，用贪婪模式匹配commands
或显式声明参数
lec [command]* = [param]*



lec [command]* -[option]+ [param]*
lec [command]* (--option [param]*)+  = [param]*

---------
每个命令下都有
	*个子命令
	*个选项
	*个参数


lec node_mirror set xxxxx
lec node_mirror
lec init -m http://cnpm.com -d ~/.config/lvs
lec init -m=http://cnpm.com -d=~/.config/lvs
# 不用=效果更好

lec mk -wosxS /wo/wo /ab/c
cp /wo/a /wo/b -o /home


lec [param]*
lec [command]+ [param]* //这种情况下，用贪婪模式匹配commands
或显式声明参数
lec [command]+ = [param]*


lec [command]* (option [param]*)+  = [param]*

cp a b c -o /home/
cp =a b c -o /home/

(=) # ()里面可省略，但是，在参数名字正好和子命令一样导致冲突时，必须要写。
lec (=)[param]+  (option [param]*)*  (=)[param]*
lec [command]*   (option [param]*)*  (=)[param]*

```
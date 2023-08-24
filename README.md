# lec
A library for creating powerful modern CLI applications. 一个用于创建功能强大的现代CLI应用程序的库

```js
lvs 命令主体

command 子命令 只能由 大小写字母、数字、中划线、下划线 组成

param 参数 只能由 大小写字母、数字、中划线、下划线 组成
config 配置开关，配置开关有长命令和短命令，
短命令 只能由 大小写字母 组成
长命令 只能由 大小写字母、数字、中划线、下划线 组成

短命令不能有参数
长命令可以有参数

每个配置开关可以有 *个参数
示例：
-h --help			//简洁介绍
-H --helps		 //详细介绍，当没有详细介绍时，显示简洁介绍， 后面可以带多个参数，进行多 关键词搜索
-v --version



lvs [command]* [param]* //这中情况下，用贪婪模式匹配commands
或显式声明参数
lvs [command]* = [param]*



lvs [command]* -[config]+ [param]*
lvs [command]* (--config [param]*)+

---------
每个命令下都有
	*个子命令
	*个配置开关
	*个参数

















```
### Todo-rs
一个基于rust的命令行todo列表工具

开发日志：https://mrtowc.xlog.app/1Todo-rs

#### 借鉴资料
jemalloc 内存分配器<br>
https://juejin.cn/post/7350320544526516263  <br>
https://www.cnblogs.com/RioTian/p/18970973  <br>
comfy-table 表格输出  <br>
https://crates.io/crates/comfy-table  <br>
将终端搬入浏览器  <br>
https://zellij.dev/news/web-client-multiple-pane-actions/  <br>

#### 简易展示
![alt text](7fcf0e2fd5f2089b5c2d3b6604921387.png)

#### 快捷指令
'help'将显示帮助信息  <br>
'exit'/'quit'/'q'将退出程序  <br>
'clear'清空控制台  <br>
'sysinfo'将显示系统信息  <br>
'add'/'del'/'edit'/'list'将显示相应信息  <br>
'/done id'、'/undone id'将任务状态改为完成或改错了，再改为未完成  <br>
'/group done'  <br>
'/save'备份数据  <br>
'/calendar' 显示日历？  <br>

#### 计划项目功能
- 任务完成类型：<br>
每天|每周|每月|每年|X次|指定日期前完成
- 任务级别：普通/重要/紧急
- 多窗口与托盘化运行：<br>
可以在多个窗口中查看和管理任务，例如查看指定组的任务详情，就会在新窗口中显示
- 任务emoji美化输出：<br>
任务名前显示emoji符号，显示任务状态。<br>
红/黄/绿 字体颜色： 任务级别(一般/重要/紧急)<br>
字体画删除线：任务完成状态<br>
单任务/整个组内任务完成就画一条删除线。<br>
最后显示截至日期与剩余天数
- 编辑时，若输入内容为空，则编辑不修改原内容<br>
- 按关键词查找任务<br>
- 表格化输出<br>
- 批量操作: <br>
    输入一次指令，编辑多个任务，比如批量完成任务。
- 完成记录<br>
    每个任务完成后，会记录下完成时间
- 日志记录<br>
    每次操作都会记录下操作时间、操作类型、操作内容<br>
    同时记录下当时所占内存、cpu占用率<br>

#### 项目优化
删除任务时不对任务ID进行重新排序<br>
或者id可以为 1-1/0，1代表第1个任务，1/0表示任务是否有效
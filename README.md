# 自动生成react项目

## 说明
- 基于`create-react-app`脚手架
- 随机生成页面内容并写入路由
- 目前只引入了`antd`和`lodash`的部分组件

## 使用
- clone该项目
- 切换至项目根目录
- 执行

    ```
    ./react-codegen -page 1 -comp 1
    ```
- 生成的页面位于`template目录`，`npm install`后即可运行

参数说明:    

- page 生成的页面数量
- comp 每个页面的组件数量
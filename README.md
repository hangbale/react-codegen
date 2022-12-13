# react测试项目生成

## 说明
- 可生成大体积的项目，用于测试打包工具的性能
- 生成的代码基于`create-react-app`脚手架
- 页面中会随机使用了`antd`和`lodash`的部分组件

## 使用
- clone该项目
- 切换至项目根目录
- 执行
   - 非windows
    ```
    ./react-codegen -page 1 -comp 1
    ```
    - windows
    需自行编译rust代码后使用
- 前端代码位于`template目录`，`yarn install`后即可运行

参数说明:    

- page 生成的页面数量
- comp 每个页面的组件数量(目前只包含了antd和lodash的组件)

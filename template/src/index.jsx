import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import routes from './routes';
import zhCN from 'antd/es/locale/zh_CN';
import { ConfigProvider } from 'antd';
import { RouterProvider } from "react-router-dom";

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <ConfigProvider locale={zhCN}>
    <RouterProvider router={routes} />
  </ConfigProvider>
);

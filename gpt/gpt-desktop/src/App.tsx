import { FC, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { LaptopOutlined, NotificationOutlined, UserOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Breadcrumb, Layout, Menu, theme } from 'antd';
import React from "react";
import { getI18n } from "./utils/i18";
import { GlobalConfig } from "./interfaces";
import GlobalContext from "./contexts/global";
import ChatPage from "./pages/chat";

const { Header, Content, Footer, Sider } = Layout;

// document.addEventListener('DOMContentLoaded', () => {
//     // This will wait for the window to load, but you could
//     // run this function on whatever trigger you want
//     setTimeout(() => {
//         invoke('close_splashscreen')
//     }, 5000) // 5000 milliseconds = 5 seconds
// })

interface SelectItem {
    key: string,
    icon: React.ReactElement,
    label: string,
    content: React.ReactElement,
}

const selectItems: SelectItem[] = [
    {
        key: 'Chat',
        icon: <UserOutlined />,
        label: 'Chat',
        content: <ChatPage />
    },
    {
        key: 'Chat1',
        icon: <UserOutlined />,
        label: 'Chat1',
        content: <ChatPage />
    },
];

const App: FC = () => {
    const [configs, setConfigs] = useState<Partial<GlobalConfig>>({});
    const i18n = getI18n(configs.lang ?? 'en');

    const defaultSelectedKeys = 'Chat'

    const [selectedKey, setSelectedKey] = useState(defaultSelectedKeys)


    return (
        <GlobalContext.Provider
            value={{
                i18n,
            }}
        >
            <Layout>
                <Menu
                    theme="dark"
                    mode="horizontal"
                    defaultSelectedKeys={[defaultSelectedKeys]}
                    onSelect={({ key }) => {
                        setSelectedKey(key)
                    }}
                    items={selectItems.map(({ key, icon, label }) => {
                        return {
                            key: key,
                            icon: icon,
                            label: label,
                        }
                    })}
                />
                <Content>
                    {
                        selectItems.map((item) => {
                            return (item.key === selectedKey && item.content)
                        })
                    }
                </Content>
            </Layout >
        </GlobalContext.Provider>
    );
};

export default App;

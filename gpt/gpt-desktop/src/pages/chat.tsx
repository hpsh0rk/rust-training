import { Layout } from "antd";

import { useState, useEffect, useContext } from "react";
import ChatContext from "../contexts/chat";
import GlobalContext from "../contexts/global";
import type { Conversation, RecordCardItem } from "../interfaces";
import Content from "../modules/Content";
import Sidebar from "../modules/Sidebar";


const ChatPage: React.FC = () => {
    const { i18n } = useContext(GlobalContext);

    const [currentId, setCurrentId] = useState<string>('1');
    const [conversations, setConversations] = useState<
        Record<string, Conversation>
    >({
        ['1']: {
            id: '1',
            messages: [],
            createdAt: Date.now(),
            title: i18n.status_empty!,
        },
    });

    const list = Object.values(conversations)
        .reverse()
        .map((conversation) => ({
            key: conversation.id,
            title: conversation.title,
            message: conversation.messages.slice(-1)?.[0]?.content ?? '',
            messageSize: conversation.messages.length,
            time:
                conversation.messages.slice(-1)?.[0]?.createdAt ??
                conversation.createdAt,
        }));

    const getSidebar = () => <Sidebar data={list} />;
    const getContent = () => <Content data={list} />;

    return (
        <ChatContext.Provider
            value={{
                currentId,
                setCurrentId,
                conversations,
                setConversations,
            }}
        >
            <div
                className={`w-[100%] h-[100%] flex overflow-hidden 'rounded-2xl'`}
                style={{ boxShadow: '0 20px 68px rgba(0, 0, 0, 0.15)' }}
            >
                <>
                    <div className="w-1/3 ">{getSidebar()}</div>
                    <div className="w-2/3 flex">
                        <div
                            className={`h-full 'w-full' flex-1`}
                        >
                            {getContent()}
                        </div>
                    </div>
                </>
            </div>
        </ChatContext.Provider>
    );
};

export default ChatPage;

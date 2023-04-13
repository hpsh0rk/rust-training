
import { FC, useContext, useState } from "react";
import MessageBox from "../../components/MessageBox";
import ChatContext from "../../contexts/chat";
import { Message, RecordCardItem } from "../../interfaces";
import MessageInput from "./MessageInput";
import { invoke } from "@tauri-apps/api/tauri";

const Content: FC<{
    data: RecordCardItem[];
}> = ({ data }) => {
    // input text
    const [text, setText] = useState('');
    const [streamMessageMap, setStreamMessageMap] = useState<
        Record<string, string>
    >({});
    const [loadingMap, setLoadingMap] = useState<Record<string, boolean>>({});

    const { currentId, conversations, setConversations } =
        useContext(ChatContext);

    const conversation = conversations[currentId];
    const messages = conversation?.messages ?? [];
    const stremMessage = streamMessageMap[currentId] ?? '';
    const loading = loadingMap[currentId];

    const updateMessages = (msgs: Message[]) => {
        setConversations((msg) => ({
            ...msg,
            [currentId]: {
                ...conversations[currentId],
                messages: msgs,
                ...(msgs.length > 0
                    ? {
                        title: msgs[0].content,
                    }
                    : {}),
            },
        }));
    };

    // todo
    // 看看是不是统一一个地方放和rust交互的接口
    // markdown修复
    // 使用流的方式进行交互。
    // xuke项目上新
    const sendTextChatMessages = async (content: string) => {
        const current = currentId;
        const input: Message[] = [
            {
                role: 'user',
                content,
                createdAt: Date.now(),
            },
        ];
        const allMessages: Message[] = messages.concat(input);
        updateMessages(allMessages);
        setText('');
        setLoadingMap((map) => ({
            ...map,
            [current]: true,
        }));

        invoke<string>("chat_with_messages", { messages: allMessages }).then((result) => {
            updateMessages(
                allMessages.concat([
                    {
                        role: 'assistant',
                        content: result,
                        createdAt: Date.now(),
                    },
                ])
            );
        }).catch((e) => {
            updateMessages(
                allMessages.concat([
                    {
                        role: 'assistant',
                        content: `Error: ${e.message || e.stack || e}`,
                        createdAt: Date.now(),
                    },
                ])
            );
        })

        setLoadingMap((map) => ({
            ...map,
            [current]: false,
        }));
    };
    // async function chatWithMessages(context: string) {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //
    //     const NewMsg: Message = { role: "user", content: context }
    //
    //     const updateMsg: Message[] = [...messages, NewMsg];
    //
    //     setMessages(updateMsg)
    //
    //     invoke<string>("chat_with_messages", { messages: updateMsg }).then((result) => {
    //         // setGptMsg(result);
    //         const gptMsg: Message = { role: "assistant", content: result }
    //         setMessages((msg) => ([
    //             ...msg,
    //             gptMsg,
    //         ]))
    //     }).catch((error) => console.error(error))
    // }

    return (
        <div className="flex flex-col h-full w-full">
            <div className="w-full h-[60px] flex items-center justify-between pl-5 pr-5 border-b border-b-[#edeeee] overflow-hidden">
                <div className="flex items-center flex-1 overflow-hidden">
                    <div className="text-[#232629] flex-1 truncate mr-2">
                        {conversation.title}
                    </div>
                </div>
            </div>
            <div className="flex-1 overflow-auto common-scrollbar p-5 pb-0">
                <MessageBox
                    streamMessage={stremMessage}
                    messages={messages}
                    loading={loading}
                />
            </div>
            <MessageInput
                text={text}
                setText={setText}
                currentId={currentId}
                onSubmit={async (message: string) => {
                    sendTextChatMessages(message);
                }}
                loading={loading}
            />
        </div>
    )
}

export default Content;

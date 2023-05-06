import { throttle } from 'lodash-es';
import { FC, useCallback, useContext, useEffect } from 'react';
import GlobalContext from '../../contexts/global';
import { Message } from '../../interfaces';
import { getRelativeTime } from '../../utils/data';
import markdown from '../../utils/markdown';
import './index.css';

const MessageItem: FC<{ message: Message; index?: number }> = ({
    message,
    index,
}) => {

    const createdAt = getRelativeTime(message.createdAt, true);
    return (
        <div
            className={`msg-fade-in flex items-start relative ${index === 0 ? '' : 'mt-[24px]'
                } ${message.role === 'user' ? 'flex-row-reverse' : ''}`}
        >
            <div
                dangerouslySetInnerHTML={{
                    __html: markdown.render(message.content),
                }}
                className={`prose message-box shadow-sm p-4 ${message.role === 'user' ? 'bg-teal-50' : 'bg-[#ebeced]'
                    } break-words overflow-hidden rounded-[16px]`}
            />
            {createdAt ? (
                <div
                    className={`message-box-time invisible text-[#a1a7a8] text-sm absolute top-[-20px] ${message.role === 'user' ? 'right-0' : 'left-[calc(3px+0.5rem)]'
                        }`}
                >
                    {createdAt}
                </div>
            ) : null}
        </div>
    );
};

const MessageBox: FC<{
    streamMessage: string;
    messages: Message[];
    loading: boolean;
}> = ({ streamMessage, messages, loading }) => {
    const { i18n } = useContext(GlobalContext);

    const handleAutoScroll = useCallback(
        throttle(() => {
            const element = document.querySelector('#content')!;
            element.scrollIntoView({
                behavior: 'smooth',
                block: 'end',
            });
        }, 300),
        []
    );

    useEffect(() => {
        handleAutoScroll();
    }, [streamMessage]);

    useEffect(() => {
        const clock = setTimeout(() => {
            handleAutoScroll();
        }, 300);

        return () => {
            clearTimeout(clock);
        };
    }, [messages]);

    return (
        <div id="content" className="pb-5">
            {messages.length === 0 ? (
                <div
                    className="prose text-gray-500 mb-[20px]"
                    dangerouslySetInnerHTML={{
                        __html: markdown.render(
                            i18n.default_text_tips
                        ),
                    }}
                />
            ) : null}
            {messages.map((message, index) => (
                <MessageItem key={index} index={index} message={message} />
            ))}
            {loading ? (
                <div className="loading text-center text-gray-400 mt-5 mb-5">
                    {i18n.status_loading}
                </div>
            ) : null}
        </div>
    );
};

export default MessageBox;

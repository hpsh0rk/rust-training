import { FC, useContext, useState } from 'react';
import { Dropdown, Input, MenuProps } from 'antd';
import { omit, sortBy } from 'lodash-es';
// import './index.css';
import RecordCard from './RecordCard';
import GlobalContext from '../../contexts/global';
import ChatContext from '../../contexts/chat';
import { getMaxIndex } from '../../utils';
import { RecordCardItem } from '../../interfaces';

const Sidebar: FC<{
    data: RecordCardItem[];
}> = ({ data }) => {
    const [keyword, setKeyword] = useState('');
    const { i18n } = useContext(GlobalContext);
    const { currentId, setCurrentId, conversations, setConversations } = useContext(ChatContext);

    const onAdd = () => {
        const id = getMaxIndex(data).toString();
        setConversations((items) => ({
            ...items,
            [id]: {
                id,
                title: i18n.status_empty!,
                messages: [],
                createdAt: Date.now(),
            },
        }));
        setCurrentId(id);
    };

    const onDelete = (key: string) => {
        setConversations((items) => omit(items, [key]));
        // delete other conversation doesnt need to update currentTab
        if (currentId === key) {
            setCurrentId(data.filter((tab) => tab.key !== key)[0]?.key);
        }
    };

    const items: MenuProps['items'] = [
        {
            key: '1',
            label: (
                <div >
                    <i className="ri-chat-4-line align-bottom mr-1" />
                    {i18n.chat_mode_text}
                </div>
            ),
        },
    ];

    const filterData = data.filter(
        (item) =>
            item.title.includes(keyword) ||
            conversations[item.key]?.messages?.some((message) =>
                message.content.includes(keyword)
            )
    );

    return (
        <div className="flex flex-col h-full">
            <div className="p-2 flex items-center justify-between mb-4">
                <div className="rounded-xl h-10 border flex-1">
                    <Input
                        className="h-[100%]"
                        value={keyword}
                        onChange={(e) => setKeyword(e.target.value)}
                        prefix={<i className="ri-search-2-line" />}
                        placeholder={i18n.search_placeholder}
                        bordered={false}
                        allowClear
                    />
                </div>
                <div>
                    <svg onClick={() => onAdd()} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                        <path className="ri-chat-new-line cursor-pointer p-2 ml-1" strokeLinecap="round" strokeLinejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>

            </div>
            <div className="common-scrollbar flex-1 p-2 pt-0 overflow-auto">
                {sortBy(filterData, ['time'])
                    .reverse()
                    .map((conversation, index) => (
                        <div key={conversation.key}>
                            {index !== 0 ? (
                                <div className="h-[1px] ml-2 mr-2" />
                            ) : null}
                            <RecordCard
                                data={conversation}
                                selected={conversation.key === currentId}
                                onSelect={() => { console.log(currentId); setCurrentId(conversation.key) }}
                                onDelete={
                                    () => onDelete(conversation.key)
                                }
                            />
                        </div>
                    ))}
            </div>
        </div>
    );
};

export default Sidebar;

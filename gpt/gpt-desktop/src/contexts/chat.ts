import { createContext } from 'react';
import { noop } from 'lodash-es';
import { I18n } from '../utils/i18';
import { Conversation, GlobalConfig, ReactSetState } from '../interfaces';

const ChatContext = createContext<{
    currentId: string;
    setCurrentId: ReactSetState<string>;
    conversations: Record<string, Conversation>;
    setConversations: ReactSetState<Record<string, Conversation>>;
}>({
    currentId: '1',
    setCurrentId: noop,
    conversations: {},
    setConversations: noop,
});
export default ChatContext;

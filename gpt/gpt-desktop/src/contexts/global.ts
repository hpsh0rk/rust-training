import { createContext } from 'react';
import { noop } from 'lodash-es';
import { I18n } from '../utils/i18';
import { Conversation, GlobalConfig, ReactSetState } from '../interfaces';

const GlobalContext = createContext<{
    i18n: Partial<I18n>;
    // configs: Partial<GlobalConfig>;
    // isMobile: boolean;
    // currentId: string;
    // setCurrentId: ReactSetState<string>;
    // conversations: Record<string, Conversation>;
    // setConversations: ReactSetState<Record<string, Conversation>>;
}>({
    i18n: {},
    // configs: {},
    // isMobile: false,
    // currentId: '1',
    // setCurrentId: noop,
    // conversations: {},
    // setConversations: noop,
});
export default GlobalContext;

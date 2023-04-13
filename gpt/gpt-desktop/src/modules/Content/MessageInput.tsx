import {
    FC,
    useCallback,
    useContext,
    useEffect,
    useRef,
    useState,
} from 'react';
import { Input } from 'antd';
import GlobalContext from '../../contexts/global';
import PromptSelect from '../../components/PromptSelect';
import React from 'react';


const MessageInput: FC<{
    text: string;
    setText: (text: string) => void;
    currentId: string; // current conversation id
    onSubmit: (message: string) => Promise<void>;
    loading: boolean;
}> = ({
    text,
    setText,
    currentId,
    onSubmit,
    loading,
}) => {
        const { i18n } = useContext(GlobalContext);
        const [promptKeyword, setPromptKeyword] = useState('');
        const [isInputComposition, setIsInputComposition] = useState(false);

        // prompt
        const [showPrompt, setShowPrompt] = useState(false);

        // textarea ref
        const ref = useRef<HTMLInputElement | null>(null);

        const disabled = text.trim() === '' || loading;

        const handleSubmit = () => {
            if (disabled) return;
            onSubmit(text);
        };

        const onPromptSelect = useCallback(
            (prompt: string) => {
                setShowPrompt(false);
                setTimeout(() => {
                    setText(prompt);
                    ref.current?.focus();
                }, 400);
            },
            [setShowPrompt, setText]
        );

        useEffect(() => {
            if (showPrompt) {
                ref.current?.focus();
            }
        }, [showPrompt]);

        useEffect(() => {
            // auto focus when change conversation
            ref.current?.focus();
        }, [currentId]);

        return (
            <div className="flex items-center p-5 pt-0">
                <PromptSelect
                    keyword={promptKeyword}
                    showPrompt={showPrompt}
                    onSelect={onPromptSelect}
                >
                    <div className="flex-1 border border-[#dfdfdf] rounded-lg relative">
                        <Input.TextArea
                            ref={ref}
                            placeholder={i18n.chat_placeholder}
                            value={text}
                            autoFocus
                            bordered={false}
                            onChange={(event) => {
                                const val = event.target.value;
                                setText(val);
                                if (val.startsWith('/')) {
                                    setShowPrompt(true);
                                    setPromptKeyword(val.slice(1));
                                } else {
                                    setShowPrompt(false);
                                    setPromptKeyword('');
                                }
                            }}
                            onCompositionStart={() => setIsInputComposition(true)}
                            onCompositionEnd={() => setIsInputComposition(false)}
                            onPressEnter={(e) => {
                                if (!e.shiftKey && !isInputComposition) {
                                    e.preventDefault();
                                    handleSubmit();
                                }
                            }}
                            size="large"
                            autoSize={{ minRows: 1, maxRows: 38 }}
                            allowClear
                        />
                    </div>
                </PromptSelect>

                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                    className={`${disabled ? 'cursor-not-allowed' : 'cursor-pointer'
                        } text-gradient text-[24px] ml-[0.5rem] ri-send-plane-fill w-6 h-6`}
                    onClick={() => handleSubmit()}
                >
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                </svg>
            </div>
        );
    };

export default MessageInput;

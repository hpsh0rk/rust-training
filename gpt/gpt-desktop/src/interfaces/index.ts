import { SupportedImgSize, SupportedModel } from "../configs";

export interface Message {
    content: string;
    role: 'assistant' | 'user';
    createdAt: number;
}

export type Lang = 'zh' | 'en';

export interface Conversation {
    id: string;
    title: string;
    messages: Message[];
    createdAt: number;
    updatedAt?: number;
}

export interface GlobalConfig {
    openAIApiKey: string;
    model: SupportedModel;
    save: boolean;
    continuous: boolean;
    messagesCount: number;
    temperature: number;
    imagesCount: number;
    imageSize: SupportedImgSize;
    lang: Lang;
}

export interface Prompt {
    act: string;
    prompt: string;
}

export interface RecordCardItem {
    key: string;
    title: string;
    message: string; // last message
    time?: number; // last message time
    messageSize: number;
}

export type ReactSetState<T> = React.Dispatch<React.SetStateAction<T>>;

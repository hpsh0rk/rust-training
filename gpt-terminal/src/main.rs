mod openai;

/**
已知问题：
- 浏览器用不了，没有tauri的window必报错，暂时没找到解决方案
- code不能漂亮的展示。不知道是不是前端markdown那个库需要再配置什么
- 切换sidebar，怎样才能展示不同的component，还是一定要用路由？


*/

/**
前后端怎么分工?
目前这种架构，其实只有前端，没有一个远程的服务器。

ts只负责交互逻辑。
rust负责数据和处理逻辑。
*/

// ------------------------------ 可能用到的库
// 前端
//  markdown在线预览：https://github.com/hinesboy/mavonEditor
//  可视化表单：https://github.com/json-editor/json-editor

// --------------------------------------------- TODO ------------------------
/*
4.10-4.12
重构现有代码，不合理的进行调整。有些前端写死的，应该让后端传。key应该前端传->用户配置怎么维护
配置相关
    模型选择
    是否连续对话&携带历史消息数
    其他参数？比如发散程度。这里其实要定义，哪些模型支持哪些参数，然后告诉前端去渲染了

Chat{

}

4.13-
prompt相关
后端：
    增删改查。
    关注功能。
    支持从不同的源导入(配置落库？定时更新？)。并且前端可以写解析脚本，解析同步格式&文件的数据，然后保存下来用
        https://github.com/f/awesome-chatgpt-prompts/blob/main/prompts.csv
        https://github.com/PlexPt/awesome-chatgpt-prompts-zh/blob/main/prompts-zh-TW.json
        https://github.com/PlexPt/awesome-chatgpt-prompts-zh/blob/main/prompts-zh.json
        https://github.com/timqian/openprompt.co/blob/main/TopPrompts.json
前端：
    prompt增删改查页面。
    关注功能。方便做筛选。
    支持填写源导入，写解析脚本。


prompt的结构体
{
type: string,  // system，chat
               // system是最开始就能发送给gpt的，这种应该可以弄成卡片展示。
               // chat是聊天过程中可以方便输入的东西
title: string,  // 展示的标题
lang: string,   // 什么语言？是否有必要？
templete: string, // 模板
var: map[string]string   //模板里面的变量

catalog: string  // 分类，方便检索，比如编程，画流程图，头脑风暴等。参考：https://dev.clickprompt.org/zh-CN/chatgpt-general/
star: bool   // 是否关注
last_use: date() // 最后一次使用时间，方便做筛选，应该要有历史数据才能做。
history: 关联的执行历史数据。
}

提供通用追加功能。比如“不知道请回答不知道”

参考notion里面收藏的其他prompt，发散思路。

历史记录维护。保存。


pipline

支持其他平台
    文心一言
    bing
        https://github.com/jlvihv/BingGPT
    https://www.perplexity.ai/
    cursor（看看能不能抓到接口，可能比较难）

其他模型？比如huggingFace里面的东西结合？提供插件功能，方便接入其他模型？
集成语音模型，作为一个外教app？
    ->  延伸下去，应该提供高度插件定制化的能力，多个模型&多个组件编排结合，形成一个app。
    ->  可能要调研下huggingFace，多模型能力做到什么程度了，按道理

*/

// --------------------------------------------- 不急的TODO ------------------------
/*
提供远程服务器。
云端同步，数据共享。

用户登录功能。
- prompt上传后，所有权的问题。
- 历史记录保存。支持本地保存/同步到云端。
- 可以把相关配置同步到云端。

用户组。
- 共享prompt，共享token，历史记录等？
 */

// i18n

/*
插件系统，与第三方平台交互

语音输入，和gpt交互，gpt自动写代码并且部署，应该通过插件系统就能做到。  但是现在想不懂他们是怎么交互的？？
 */

// 支持chrome标签页管理
//  https://betterprogramming.pub/creating-chrome-extensions-with-typescript-914873467b65
//  https://github.com/chibat/chrome-extension-typescript-starter

/*
rustup default 1.62.0
 */
fn main() {
    println!("Hello, world!");
}

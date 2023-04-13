# 注意
1. gpt/src/openai/client里面写死了openai的token，token会过期，记得改
2. 发送消息是同步的，需要修改gpt-desktop/src-tauri/src/main.rs中的方法，改为async的，但是会报错，暂时找不到解决方式，对rust异步这块不了解，还是太菜了呀
3. Chat页面聊天完，切换到Chat1会导致Chat的页面丢失

# 运行&编译
`cargo tauri dev`
`cargo tauri build`

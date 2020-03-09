// 顶一个一个枚举体
enum WebEvent {
    // 可以是一个单元结构体
    PageLoad,
    PageUnload,
    // 或者一个元组结构体
    KeyPress(char),
    Paste(String),
    // 或者一个普通结构体
    Click { x: i32, y: i32}
}

fn inspect(_event: WebEvent) {
    // match 表达式
    match _event {
        WebEvent::PageLoad => println!("Page Loaded."),
        WebEvent::PageUnload => println!("Page didn't load."),
        // 从枚举体中解构出字符
        WebEvent::KeyPress(ch) => println!("Press '{}' key.", ch),
        WebEvent::Paste(st) => println!("Paste \"{}\" sentence.", st),
        // 从结构体中解构出变量
        WebEvent::Click { x, y } => {
            println!("clicked at x = {}, y = {}.", x, y);
        },
    }
}

fn main() {
    // 分别创建枚举体中的值
    let page_load = WebEvent::PageLoad;
    let page_unload = WebEvent::PageUnload;
    let press = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("Hello World!!!".to_owned());
    let click = WebEvent::Click { x: 20, y: 20 };

    inspect(page_load);
    inspect(page_unload);
    inspect(press);
    inspect(paste);
    inspect(click);
}
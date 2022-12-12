mod preset;
mod gen;
fn main() {
    let mut page_count = 20;
    let mut component_per_page = 20;
    let args = std::env::args().collect::<Vec<String>>();
    let has_page = args.iter().position(|x| x == "-page");
    let has_component = args.iter().position(|x| x == "-comp");
    if let Some(page) = has_page {
        if (page + 1) >= args.len() {
            println!("page 参数错误");
            return;
        }
        page_count = args[page + 1].parse::<u8>().expect("page 参数错误");
    }

    if let Some(component) = has_component {
        if (component + 1) >= args.len() {
            println!("comp 参数错误");
            return;
        }
        component_per_page = args[component + 1].parse::<u8>().expect("comp 参数错误");
    }
    let mut pages: Vec<(String, String)> = Vec::new();
    for i in 0..page_count {
        let page = gen::gen_page(component_per_page, i);
        pages.push(page);
    }
    gen::gen_router(pages);
    println!("✅ 执行成功");
}

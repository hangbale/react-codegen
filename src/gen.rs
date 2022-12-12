use super::preset::{get_str, Route, get_lodash_str};
use rand::Rng;


fn gen_render(fncalls: &str,content: &str) -> String {
    let a = r#"
    {fncalls}
    return (
        <div>
            {content}
        </div>
    );
    "#;
    let r = a.replace("{content}", content);
    r.replace("{fncalls}", fncalls)
}
fn gen_function(fn_name: &str, content: &str) -> String {
    let a = r#"
    export default function {fnName}() {
        {content}
    }
    "#;
    let t = a.replace("{content}", content);
    t.replace("{fnName}", fn_name)
}

pub fn gen_page(component_num: u8, page_index: u8) -> (String, String) {
    let imports = get_str(component_num);
    let lodash = get_lodash_str(component_num);
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..9999);
    let file_name = format!("page_{}_{}", page_index, index);
    let mut content = imports.1;
    let mut import_str = imports.0;
    import_str = format!("{}\n{}", import_str, lodash.0);

    let render_str = gen_render(&lodash.1, &content);
    let function_str = gen_function(&file_name, &render_str);
    let file_str = format!("{}\n{}", import_str, function_str);

    std::fs::write(format!("template/src/pages/{}.jsx", &file_name), file_str).unwrap();
    (file_name.clone(), format!("./pages/{}.jsx", file_name))
}

pub fn gen_router(pages: Vec<(String, String)>) {
    let mut imports = String::new();
    let mut routes = String::new();

    for page in pages {
        let config = r#"
        {
            path: '/{path}',
            element: <{component} />,
        },
        "#;
        let b = config.replace("{path}", &page.0);
        let comp_name = page.0.replace("p", "P");
        let c = b.replace("{component}", &comp_name);
        routes = format!("{}\n{}", routes, c);
        let import = r#"import {component} from "{path}";"#;
        let d = import.replace("{component}", &comp_name);
        let e = d.replace("{path}", &page.1);
        imports = format!("{}\n{}", imports, e);
    }
    let a = Route.replace("{imports}", &imports);
    let b = a.replace("{routesList}", &routes);
    std::fs::write("template/src/routes.jsx", b).unwrap();
}
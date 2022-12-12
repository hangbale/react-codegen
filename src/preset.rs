use rand::Rng;

const Antd: [(&str, &str); 9] = [
    (r#"import { Input } from "antd";"#, r#"<Input placeholder="请输入" />"#),
    (r#"import { Button } from "antd";"#, r#"<Button type="primary">Hello World</Button>"#),
    (r#"import { Rate } from "antd";"#, r#"<Rate allowHalf defaultValue={2.5} />"#),
    (r#"import { Slider } from "antd";"#, r#"<Slider range defaultValue={[20, 50]} disabled={false} />"#),
    (r#"import { Empty } from "antd";"#, r#"<Empty />"#),
    (r#"import { Tag } from "antd";"#, r#"<Tag>Tag 1</Tag>"#),
    (r#"import { Spin } from "antd";"#, r#"<Spin />"#),
    (r#"import { Alert } from "antd";"#, r#"<Alert message="Success Text" type="success" />"#),
    (r#"import { Result } from "antd";"#, r#"<Result
    status="success"
    title="Successfully Purchased Cloud Server ECS!"
    subTitle="Order number: 2017182818828182881 Cloud server configuration takes 1-5 minutes, please wait."
    />"#),
];

const Lodash: [(&str, &str); 9] = [
    (r#"import { map } from "lodash";"#, r#"map([1, 2, 3], (n) => n * 3);"#),
    (r#"import { filter } from "lodash";"#, r#"filter([1, 2, 3, 4, 5], (n) => n % 2 == 0);"#),
    (r#"import { find } from "lodash";"#, r#"find([1, 2, 3, 4, 5], (n) => n % 2 == 0);"#),
    (r#"import { findIndex } from "lodash";"#, r#"findIndex([1, 2, 3, 4, 5], (n) => n % 2 == 0);"#),
    (r#"import { findLast } from "lodash";"#, r#"findLast([1, 2, 3, 4, 5], (n) => n % 2 == 0);"#),
    (r#"import { findLastIndex } from "lodash";"#, r#"findLastIndex([1, 2, 3, 4, 5], (n) => n % 2 == 0);"#),
    (r#"import { forEach } from "lodash";"#, r#"forEach([1, 2, 3, 4, 5], (n) => console.log(n));"#),
    (r#"import { forEachRight } from "lodash";"#, r#"forEachRight([1, 2, 3, 4, 5], (n) => console.log(n));"#),
    (r#"import { includes } from "lodash";"#, r#"includes([1, 2, 3, 4, 5], 3);"#),
];

pub const Route: &str = r#"
import { createBrowserRouter } from "react-router-dom";
{imports}
const config = [
{
    path: '/',
    element: (
        <div>
            this is home page
        </div>
    ),
},
{routesList}
]

export default createBrowserRouter(config);
    "#;

pub fn get_str(count: u8) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut result: (String, String) = ("".to_owned(), "".to_owned());
    let import_all = Antd.iter().map(|x| x.0).collect::<Vec<&str>>().join("\n");
    result.0 = import_all;
    for _ in 0..count {
        let index = rng.gen_range(0..9);
        let t = Antd[index];
        result.1 = format!("{}\n{}", &result.1, t.1);
    }
    result
}

pub fn get_lodash_str(count: u8) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut result: (String, String) = ("".to_owned(), "".to_owned());
    let import_all = Lodash.iter().map(|x| x.0).collect::<Vec<&str>>().join("\n");
    result.0 = import_all;
    for _ in 0..count {
        let index = rng.gen_range(0..9);
        let t = Lodash[index];
        result.1 = format!("{}\n{}", &result.1, t.1);
    }
    result
}

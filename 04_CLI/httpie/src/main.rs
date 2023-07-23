use clap::{AppSetting,clap};
#[derive(Clap,Debug)]
#[clap(version = "1.0",author = "Yao<yaohii@foxmail.com>")]
#[clap(setting = AppSetting::ColoredHelp)]
struct Opts{
    #[clap(subcommand)]
    subcmd:SubCommand,
}

//暂时支持的http方法
#[derive(Clap,Debug)]
enum SubCommand{
    Get(Get),
    Post(Post),
}


#[derive(Clap,Debug)]
struct Get{
    url:String,
}

#[derive(Clap,Debug)]
struct Post{
    url:String,
    body:Vec<String>,
}

fn main() {
    let opts:Opts = Opts::parse();
    println!("{:?}",opts);
}

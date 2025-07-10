use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main(){

    let arg: Vec<String> = env::args().collect();
    create_express_route(arg.to_vec());

}

fn create_template_commands() {

    let mut is_running = false;

    while !is_running {

        let routes = vec!["--TS", "--RS", "--JS"];

        println!("{:?}", routes.to_vec());
        is_running = true;
    }

}

fn create_express_route(args: Vec<String>) {
    let mut file_created = false;


    while !file_created {

        if args.contains(&"--ARGS".to_owned()) {
            create_template_commands();
        }

        if args.contains(&"--JS".to_owned()) {
            let path = Path::new("server.js");

            if !path.exists() {
                let file = File::create("server.js");
                file.unwrap().write("".as_ref()).expect("No Input File Created");
            }
        }

        if args.contains(&"--TS".to_owned()) {
            let path = Path::new("server.ts");

            if !path.exists() {
                let file = File::create("server.ts");
                file.unwrap().write("".as_ref()).expect("No Input File Created");

            }
        }

        if args.contains(&"--RS".to_owned()) {

            let path = Path::new("main.rs");

            if !path.exists() {
                let file = File::create("main.rs");
                file.unwrap().write("".as_ref()).expect("No Input File Created");

            }

        }


        file_created = true;
    }
}

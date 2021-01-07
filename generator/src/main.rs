use core::panic;

extern crate clang;

fn print_entity(entity: &clang::Entity, level: i32) {
    for _ in 0..level {
        print!("  ");
    }

    let kind = entity.get_kind();
    print!("{:?}", kind);

    match entity.get_name() {
        Some(name) => println!("<{}>", name),
        None => println!(""),
    }
}

fn traverse(entity: &clang::Entity, level: i32) {
    print_entity(entity, level);

    for child in entity.get_children() {
        traverse(&child, level + 1);
    }
}

fn get_translation_unit<'a>(
    index: &'a clang::Index,
    headers: &[&str],
    args: &[&str],
) -> Result<clang::TranslationUnit<'a>, clang::SourceError> {
    match headers.len() {
        0 => Err(clang::SourceError::Unknown),
        1 => index.parser(&headers[0]).arguments(args).parse(),
        _ => {
            let filename = "header.h";
            let mut parser = index.parser(filename);

            let mut contents = String::new();
            for header in headers {
                contents.push_str(&std::format!("#include \"{}\"\n", header));
            }

            let unsaved = [clang::Unsaved::new(filename, contents)];
            parser.unsaved(&unsaved).arguments(args).parse()
        }
    }
}

struct CmdLine<'a> {
    headers: std::vec::Vec<&'a str>,
    args: std::vec::Vec<&'a str>,
}

impl<'a> CmdLine<'a> {
    pub fn parse(args: &'a [String]) -> CmdLine<'a> {
        let mut cmd = CmdLine {
            headers: Vec::new(),
            args: Vec::new(),
        };

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-I" => {
                    cmd.args.push("-I");
                    i += 1;
                    cmd.args.push(&args[i])
                }
                "-h" => {
                    i += 1;
                    cmd.headers.push(&args[i])
                }
                _ => panic!(),
            };
            i += 1
        }

        return cmd;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = CmdLine::parse(&args);

    let clang = clang::Clang::new().unwrap();
    let index = clang::Index::new(&clang, false, true);
    let tu = get_translation_unit(&index, &cmd.headers, &cmd.args).unwrap();

    traverse(&tu.get_entity(), 0);
}

use core::panic;

extern crate clang;

enum Decl {
    Type { name: String },
    Pointer(Box<Decl>),
    Array(Box<Decl>),
}

struct FileLocation {
    path: String,
    // public bool IsValid => !string.IsNullOrEmpty(Path.Path);

    // // text position
    // public uint Line { get; private set; }
    // public readonly uint Column;

    // // byte offset
    // public readonly int Begin;
    // public readonly int End;
}

struct TypeReference {
    location: FileLocation,
    hash: u32,
    decl: Box<Decl>,
}

struct TypeMap {
    type_map: std::collections::HashMap<u32, TypeReference>,
}

impl TypeMap {
    fn new() -> TypeMap {
        TypeMap
        {
            type_map: std::collections::HashMap::new()
        }
    }

    fn traverse(&self, entity: &clang::Entity, level: i32) {
        print_entity(entity, level);

        let kind = entity.get_kind();
        match kind
        {
            clang::EntityKind::TranslationUnit =>{}
            clang::EntityKind::InclusionDirective 
            // clang::EntityKind::ClassTemplatInclusionDirective |
            // clang::EntityKind::ClassTemplatePartialSpecializatioInclusionDirective |
            // clang::EntityKind::FunctionTemplatInclusionDirective |
            // clang::EntityKind::UsingDeclaratioInclusionDirective |
            // clang::EntityKind::StaticAsserInclusionDirective |
            // clang::EntityKind::FirstExpInclusionDirective |
            // clang::EntityKind::AlignedAttInclusionDirective |
            // clang::EntityKind::CXXAccessSpecifieInclusionDirective |
            // clang::EntityKind::ConstructoInclusionDirective |
            // clang::EntityKind::DestructoInclusionDirective |
            // clang::EntityKind::ConversionFunctioInclusionDirective
            =>
            {
                return
            }
            clang::EntityKind::MacroDefinition
            =>
            {
                // m_typeMap.ParseMacroDefinition(cursor);
                // break;
                return
            }
            clang::EntityKind::MacroExpansion
            =>{
                //     ScopedCXString spelling(clang_getCursorSpelling(cursor));
                //     if (spelling.str_view() == "DEFINE_GUID")
                //     {
                //         //   auto tokens = getTokens(cursor);
                //         //   scope(exit)
                //         //       clang_disposeTokens(tu, tokens.ptr, cast(uint)
                //         //       tokens.length);
                //         //   string[] tokenSpellings =
                //         //       tokens.map !(t = > tokenToString(cursor,
                //         //       t)).array();
                //         //   if (tokens.length == 26) {
                //         //     auto name = tokenSpellings[2];
                //         //     if (name.startsWith("IID_")) {
                //         //       name = name[4.. $];
                //         //     }
                //         //     m_uuidMap[name] = tokensToUUID(tokenSpellings[4..
                //         //     $]);
                //         //   } else {
                //         //     debug auto a = 0;
                //         //   }
                //     }
                return
            }
            clang::EntityKind::Namespace
            =>{
                // auto decl = getDecl<Namespace>(cursor);
                // if (!decl)
                // {
                //     auto hash = clang_hashCursor(cursor);
                //     auto location = Location::get(cursor);
                //     ScopedCXString spelling(clang_getCursorSpelling(cursor));
                //     decl = Namespace::create(hash, location.path(), location.line, spelling.str_view());
                //     pushDecl(cursor, decl);
                // }
                // var nested = context.Child();
                // TraverseChildren(cursor, nested);
            }
            clang::EntityKind::UnexposedDecl
            =>{
                // // ScopedCXTokens tokens(cursor);
                // var nested = context.Child();
                // // if (tokens.size() >= 2)
                // // {
                // //     // extern C
                // //     if (tokens.spelling(0).str_view() == "extern" && tokens.spelling(1).str_view() == "\"C\"")
                // //     {
                // //         child.isExternC = true;
                // //     }
                // // }
                // TraverseChildren(cursor, nested);
            }
            clang::EntityKind::TypedefDecl
            =>{
                // first
                // TraverseChildren(cursor, context);
                // var reference = m_typeMap.GetOrCreate(cursor);
                // reference.Type = TypedefType.Parse(cursor, m_typeMap);
                return
            }
            clang::EntityKind::FunctionDecl
            =>{
                // var reference = m_typeMap.GetOrCreate(cursor);
                // reference.Type = FunctionType.Parse(cursor, m_typeMap);
                return
            }
            clang::EntityKind::StructDecl |
            clang::EntityKind::ClassDecl |
            clang::EntityKind::UnionDecl 
            =>{
                // var reference = m_typeMap.GetOrCreate(cursor);
                // var structType = StructType.Parse(cursor, m_typeMap);
                // reference.Type = structType;
                // var nested = context.Enter(structType);
                // TraverseChildren(cursor, nested);
                // // if (libclang.clang_Cursor_isAnonymousRecordDecl(cursor) != 0)
                // if (libclang.clang_Cursor_isAnonymous(cursor) != 0)
                // {
                //     // anonymous type decl add field to current struct.
                //     structType.AnonymousParent = context.Current;
                //     var fieldOffset = (uint)libclang.clang_Cursor_getOffsetOfField(cursor);
                //     var current = context.Current;
                //     // var fieldName = cursor.Spelling();
                //     // FIXME: anonymous type field offset ?
                //     if (current != null)
                //     {
                //         current.Fields.Add(new StructField(current.Fields.Count, "", reference, 0));
                //     }
                //     else
                //     {
                //         var a = 0;
                //     }
                // }
            }
            clang::EntityKind::FieldDecl
            =>{
                // var fieldName = cursor.Spelling();
                // var fieldOffset = (uint)libclang.clang_Cursor_getOffsetOfField(cursor);
                // var fieldType = libclang.clang_getCursorType(cursor);
                // var current = context.Current;
                // if (!string.IsNullOrEmpty(fieldName) && current.Fields.Any(x => x.Name == fieldName))
                // {
                //     throw new Exception();
                // }
                // current.Fields.Add(new StructField(current.Fields.Count, fieldName, m_typeMap.CxTypeToType(fieldType, cursor).Item1, fieldOffset));
                // break;
                return
            }
            clang::EntityKind::BaseSpecifier
            =>{
                // var referenced = libclang.clang_getCursorReferenced(cursor);
                // var baseClass = m_typeMap.GetOrCreate(referenced);
                // context.Current.BaseClass = baseClass;
                // break;
                return
            }
            clang::EntityKind::UnexposedAttr
            =>{
                // var src = m_typeMap.GetSource(cursor);
                // if (StructType.TryGetIID(src, out Guid iid))
                // {
                //     context.Current.IID = iid;
                // }
                return
            }
            clang::EntityKind::Method
            =>{
                // var method = FunctionType.Parse(cursor, m_typeMap);
                // if (!method.HasBody)
                // {
                //     // TODO: override check

                //     // IntPtr p = default;
                //     // uint n = default;
                //     // ulong[] hashes;
                //     // libclang.clang_getOverriddenCursors(child, ref p, ref n);
                //     // if (n)
                //     // {
                //     //     scope(exit) clang_disposeOverriddenCursors(p);

                //     //     hashes.length = n;
                //     //     for (int i = 0; i < n; ++i)
                //     //     {
                //     //         hashes[i] = clang_hashCursor(p[i]);
                //     //     }

                //     //     debug
                //     //     {
                //     //         var childName = getCursorSpelling(child);
                //     //         var a = 0;
                //     //     }
                //     // }

                //     // var found = -1;
                //     // for (int i = 0; i < VTable.Count; ++i)
                //     // {
                //     //     var current = decl.vtable[i].hash;
                //     //     if (hashes.any!(x = > x == current))
                //     //     {
                //     //         found = i;
                //     //         break;
                //     //     }
                //     // }
                //     // if (found != -1)
                //     // {
                //     //     debug var a = 0;
                //     // }
                //     // else
                //     // {
                //     //     found = cast(int) decl.vtable.length;
                //     //     decl.vtable ~ = method;
                //     //     debug var a = 0;
                //     // }
                //     // decl.methodVTableIndices ~ = found;

                //     context.Current.Methods.Add(method);
                // }
                return
            }
            // break;

            clang::EntityKind::EnumDecl
            =>{
                // var reference = m_typeMap.GetOrCreate(cursor);
                // reference.Type = EnumType.Parse(cursor);
                return
            }
            // break;

            clang::EntityKind::VarDecl
            =>{
                return
            }
            // break;
            _ =>{
                panic!()
            }
        }

        for child in entity.get_children() {
            self.traverse(&child, level + 1);
        }
    }
}

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
    headers: Vec<&'a str>,
    args: Vec<&'a str>,
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

    let map = TypeMap::new();    
    map.traverse(&tu.get_entity(), 0);
}

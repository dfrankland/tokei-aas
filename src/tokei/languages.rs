use tokei::{Languages as TokeiLanguages, LanguageType as TokeiLanguageType, Language as TokeiLanguage, Stats as TokeiStats};

#[derive(Debug, Clone, juniper::GraphQLEnum)]
pub enum LanguageType {
    Abap,
    ActionScript,
    Ada,
    Agda,
    Alex,
    Asn1,
    Asp,
    AspNet,
    Assembly,
    AutoHotKey,
    Autoconf,
    Automake,
    Bash,
    Batch,
    BrightScript,
    C,
    CHeader,
    CMake,
    CSharp,
    CShell,
    Cabal,
    Cassius,
    Ceylon,
    Clojure,
    ClojureC,
    ClojureScript,
    Cobol,
    CoffeeScript,
    Cogent,
    ColdFusion,
    ColdFusionScript,
    Coq,
    Cpp,
    CppHeader,
    Crystal,
    Css,
    D,
    Dart,
    DeviceTree,
    Dockerfile,
    DotNetResource,
    DreamMaker,
    Edn,
    Elisp,
    Elixir,
    Elm,
    Elvish,
    EmacsDevEnv,
    Erlang,
    FEN,
    FSharp,
    Fish,
    Forth,
    FortranLegacy,
    FortranModern,
    Fstar,
    GdScript,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Hamlet,
    Handlebars,
    Happy,
    Haskell,
    Haxe,
    Hcl,
    Hex,
    Hlsl,
    Html,
    Idris,
    Ini,
    IntelHex,
    Isabelle,
    Jai,
    Java,
    JavaScript,
    Json,
    Jsx,
    Julia,
    Julius,
    KakouneScript,
    Kotlin,
    LLVM,
    Lean,
    Less,
    LinkerScript,
    Liquid,
    Lisp,
    Logtalk,
    Lua,
    Lucius,
    Madlang,
    Makefile,
    Markdown,
    Meson,
    Mint,
    ModuleDef,
    MsBuild,
    Mustache,
    Nim,
    Nix,
    NotQuitePerl,
    OCaml,
    ObjectiveC,
    ObjectiveCpp,
    Org,
    Oz,
    PSL,
    Pascal,
    Perl,
    Perl6,
    Pest,
    Php,
    Polly,
    Pony,
    PostCss,
    Processing,
    Prolog,
    Protobuf,
    PureScript,
    Python,
    Qcl,
    Qml,
    R,
    RPMSpecfile,
    Racket,
    Rakefile,
    Razor,
    ReStructuredText,
    Ruby,
    RubyHtml,
    Rust,
    SRecode,
    Sass,
    Scala,
    Scheme,
    Scons,
    Sh,
    Sml,
    Solidity,
    SpecmanE,
    Spice,
    Sql,
    Stratego,
    Svg,
    Swift,
    Swig,
    SystemVerilog,
    Tcl,
    Tex,
    Text,
    Toml,
    Twig,
    TypeScript,
    UnrealDeveloperMarkdown,
    UnrealPlugin,
    UnrealProject,
    UnrealScript,
    UnrealShader,
    UnrealShaderHeader,
    UrWeb,
    UrWebProject,
    VB6,
    VBScript,
    Vala,
    Verilog,
    VerilogArgsFile,
    Vhdl,
    VimScript,
    VisualBasic,
    VisualStudioProject,
    VisualStudioSolution,
    Vue,
    WebAssembly,
    Wolfram,
    XSL,
    Xaml,
    XcodeConfig,
    Xml,
    Xtend,
    Yaml,
    Zig,
    Zsh,
}

impl From<&TokeiLanguageType> for LanguageType {
    fn from(tokei_language_type: &TokeiLanguageType) -> Self {
        match tokei_language_type {
            TokeiLanguageType::Abap => LanguageType::Abap,
            TokeiLanguageType::ActionScript => LanguageType::ActionScript,
            TokeiLanguageType::Ada => LanguageType::Ada,
            TokeiLanguageType::Agda => LanguageType::Agda,
            TokeiLanguageType::Alex => LanguageType::Alex,
            TokeiLanguageType::Asn1 => LanguageType::Asn1,
            TokeiLanguageType::Asp => LanguageType::Asp,
            TokeiLanguageType::AspNet => LanguageType::AspNet,
            TokeiLanguageType::Assembly => LanguageType::Assembly,
            TokeiLanguageType::AutoHotKey => LanguageType::AutoHotKey,
            TokeiLanguageType::Autoconf => LanguageType::Autoconf,
            TokeiLanguageType::Automake => LanguageType::Automake,
            TokeiLanguageType::Bash => LanguageType::Bash,
            TokeiLanguageType::Batch => LanguageType::Batch,
            TokeiLanguageType::BrightScript => LanguageType::BrightScript,
            TokeiLanguageType::C => LanguageType::C,
            TokeiLanguageType::CHeader => LanguageType::CHeader,
            TokeiLanguageType::CMake => LanguageType::CMake,
            TokeiLanguageType::CSharp => LanguageType::CSharp,
            TokeiLanguageType::CShell => LanguageType::CShell,
            TokeiLanguageType::Cabal => LanguageType::Cabal,
            TokeiLanguageType::Cassius => LanguageType::Cassius,
            TokeiLanguageType::Ceylon => LanguageType::Ceylon,
            TokeiLanguageType::Clojure => LanguageType::Clojure,
            TokeiLanguageType::ClojureC => LanguageType::ClojureC,
            TokeiLanguageType::ClojureScript => LanguageType::ClojureScript,
            TokeiLanguageType::Cobol => LanguageType::Cobol,
            TokeiLanguageType::CoffeeScript => LanguageType::CoffeeScript,
            TokeiLanguageType::Cogent => LanguageType::Cogent,
            TokeiLanguageType::ColdFusion => LanguageType::ColdFusion,
            TokeiLanguageType::ColdFusionScript => LanguageType::ColdFusionScript,
            TokeiLanguageType::Coq => LanguageType::Coq,
            TokeiLanguageType::Cpp => LanguageType::Cpp,
            TokeiLanguageType::CppHeader => LanguageType::CppHeader,
            TokeiLanguageType::Crystal => LanguageType::Crystal,
            TokeiLanguageType::Css => LanguageType::Css,
            TokeiLanguageType::D => LanguageType::D,
            TokeiLanguageType::Dart => LanguageType::Dart,
            TokeiLanguageType::DeviceTree => LanguageType::DeviceTree,
            TokeiLanguageType::Dockerfile => LanguageType::Dockerfile,
            TokeiLanguageType::DotNetResource => LanguageType::DotNetResource,
            TokeiLanguageType::DreamMaker => LanguageType::DreamMaker,
            TokeiLanguageType::Edn => LanguageType::Edn,
            TokeiLanguageType::Elisp => LanguageType::Elisp,
            TokeiLanguageType::Elixir => LanguageType::Elixir,
            TokeiLanguageType::Elm => LanguageType::Elm,
            TokeiLanguageType::Elvish => LanguageType::Elvish,
            TokeiLanguageType::EmacsDevEnv => LanguageType::EmacsDevEnv,
            TokeiLanguageType::Erlang => LanguageType::Erlang,
            TokeiLanguageType::FEN => LanguageType::FEN,
            TokeiLanguageType::FSharp => LanguageType::FSharp,
            TokeiLanguageType::Fish => LanguageType::Fish,
            TokeiLanguageType::Forth => LanguageType::Forth,
            TokeiLanguageType::FortranLegacy => LanguageType::FortranLegacy,
            TokeiLanguageType::FortranModern => LanguageType::FortranModern,
            TokeiLanguageType::Fstar => LanguageType::Fstar,
            TokeiLanguageType::GdScript => LanguageType::GdScript,
            TokeiLanguageType::Glsl => LanguageType::Glsl,
            TokeiLanguageType::Go => LanguageType::Go,
            TokeiLanguageType::Graphql => LanguageType::Graphql,
            TokeiLanguageType::Groovy => LanguageType::Groovy,
            TokeiLanguageType::Hamlet => LanguageType::Hamlet,
            TokeiLanguageType::Handlebars => LanguageType::Handlebars,
            TokeiLanguageType::Happy => LanguageType::Happy,
            TokeiLanguageType::Haskell => LanguageType::Haskell,
            TokeiLanguageType::Haxe => LanguageType::Haxe,
            TokeiLanguageType::Hcl => LanguageType::Hcl,
            TokeiLanguageType::Hex => LanguageType::Hex,
            TokeiLanguageType::Hlsl => LanguageType::Hlsl,
            TokeiLanguageType::Html => LanguageType::Html,
            TokeiLanguageType::Idris => LanguageType::Idris,
            TokeiLanguageType::Ini => LanguageType::Ini,
            TokeiLanguageType::IntelHex => LanguageType::IntelHex,
            TokeiLanguageType::Isabelle => LanguageType::Isabelle,
            TokeiLanguageType::Jai => LanguageType::Jai,
            TokeiLanguageType::Java => LanguageType::Java,
            TokeiLanguageType::JavaScript => LanguageType::JavaScript,
            TokeiLanguageType::Json => LanguageType::Json,
            TokeiLanguageType::Jsx => LanguageType::Jsx,
            TokeiLanguageType::Julia => LanguageType::Julia,
            TokeiLanguageType::Julius => LanguageType::Julius,
            TokeiLanguageType::KakouneScript => LanguageType::KakouneScript,
            TokeiLanguageType::Kotlin => LanguageType::Kotlin,
            TokeiLanguageType::LLVM => LanguageType::LLVM,
            TokeiLanguageType::Lean => LanguageType::Lean,
            TokeiLanguageType::Less => LanguageType::Less,
            TokeiLanguageType::LinkerScript => LanguageType::LinkerScript,
            TokeiLanguageType::Liquid => LanguageType::Liquid,
            TokeiLanguageType::Lisp => LanguageType::Lisp,
            TokeiLanguageType::Logtalk => LanguageType::Logtalk,
            TokeiLanguageType::Lua => LanguageType::Lua,
            TokeiLanguageType::Lucius => LanguageType::Lucius,
            TokeiLanguageType::Madlang => LanguageType::Madlang,
            TokeiLanguageType::Makefile => LanguageType::Makefile,
            TokeiLanguageType::Markdown => LanguageType::Markdown,
            TokeiLanguageType::Meson => LanguageType::Meson,
            TokeiLanguageType::Mint => LanguageType::Mint,
            TokeiLanguageType::ModuleDef => LanguageType::ModuleDef,
            TokeiLanguageType::MsBuild => LanguageType::MsBuild,
            TokeiLanguageType::Mustache => LanguageType::Mustache,
            TokeiLanguageType::Nim => LanguageType::Nim,
            TokeiLanguageType::Nix => LanguageType::Nix,
            TokeiLanguageType::NotQuitePerl => LanguageType::NotQuitePerl,
            TokeiLanguageType::OCaml => LanguageType::OCaml,
            TokeiLanguageType::ObjectiveC => LanguageType::ObjectiveC,
            TokeiLanguageType::ObjectiveCpp => LanguageType::ObjectiveCpp,
            TokeiLanguageType::Org => LanguageType::Org,
            TokeiLanguageType::Oz => LanguageType::Oz,
            TokeiLanguageType::PSL => LanguageType::PSL,
            TokeiLanguageType::Pascal => LanguageType::Pascal,
            TokeiLanguageType::Perl => LanguageType::Perl,
            TokeiLanguageType::Perl6 => LanguageType::Perl6,
            TokeiLanguageType::Pest => LanguageType::Pest,
            TokeiLanguageType::Php => LanguageType::Php,
            TokeiLanguageType::Polly => LanguageType::Polly,
            TokeiLanguageType::Pony => LanguageType::Pony,
            TokeiLanguageType::PostCss => LanguageType::PostCss,
            TokeiLanguageType::Processing => LanguageType::Processing,
            TokeiLanguageType::Prolog => LanguageType::Prolog,
            TokeiLanguageType::Protobuf => LanguageType::Protobuf,
            TokeiLanguageType::PureScript => LanguageType::PureScript,
            TokeiLanguageType::Python => LanguageType::Python,
            TokeiLanguageType::Qcl => LanguageType::Qcl,
            TokeiLanguageType::Qml => LanguageType::Qml,
            TokeiLanguageType::R => LanguageType::R,
            TokeiLanguageType::RPMSpecfile => LanguageType::RPMSpecfile,
            TokeiLanguageType::Racket => LanguageType::Racket,
            TokeiLanguageType::Rakefile => LanguageType::Rakefile,
            TokeiLanguageType::Razor => LanguageType::Razor,
            TokeiLanguageType::ReStructuredText => LanguageType::ReStructuredText,
            TokeiLanguageType::Ruby => LanguageType::Ruby,
            TokeiLanguageType::RubyHtml => LanguageType::RubyHtml,
            TokeiLanguageType::Rust => LanguageType::Rust,
            TokeiLanguageType::SRecode => LanguageType::SRecode,
            TokeiLanguageType::Sass => LanguageType::Sass,
            TokeiLanguageType::Scala => LanguageType::Scala,
            TokeiLanguageType::Scheme => LanguageType::Scheme,
            TokeiLanguageType::Scons => LanguageType::Scons,
            TokeiLanguageType::Sh => LanguageType::Sh,
            TokeiLanguageType::Sml => LanguageType::Sml,
            TokeiLanguageType::Solidity => LanguageType::Solidity,
            TokeiLanguageType::SpecmanE => LanguageType::SpecmanE,
            TokeiLanguageType::Spice => LanguageType::Spice,
            TokeiLanguageType::Sql => LanguageType::Sql,
            TokeiLanguageType::Stratego => LanguageType::Stratego,
            TokeiLanguageType::Svg => LanguageType::Svg,
            TokeiLanguageType::Swift => LanguageType::Swift,
            TokeiLanguageType::Swig => LanguageType::Swig,
            TokeiLanguageType::SystemVerilog => LanguageType::SystemVerilog,
            TokeiLanguageType::Tcl => LanguageType::Tcl,
            TokeiLanguageType::Tex => LanguageType::Tex,
            TokeiLanguageType::Text => LanguageType::Text,
            TokeiLanguageType::Toml => LanguageType::Toml,
            TokeiLanguageType::Twig => LanguageType::Twig,
            TokeiLanguageType::TypeScript => LanguageType::TypeScript,
            TokeiLanguageType::UnrealDeveloperMarkdown => LanguageType::UnrealDeveloperMarkdown,
            TokeiLanguageType::UnrealPlugin => LanguageType::UnrealPlugin,
            TokeiLanguageType::UnrealProject => LanguageType::UnrealProject,
            TokeiLanguageType::UnrealScript => LanguageType::UnrealScript,
            TokeiLanguageType::UnrealShader => LanguageType::UnrealShader,
            TokeiLanguageType::UnrealShaderHeader => LanguageType::UnrealShaderHeader,
            TokeiLanguageType::UrWeb => LanguageType::UrWeb,
            TokeiLanguageType::UrWebProject => LanguageType::UrWebProject,
            TokeiLanguageType::VB6 => LanguageType::VB6,
            TokeiLanguageType::VBScript => LanguageType::VBScript,
            TokeiLanguageType::Vala => LanguageType::Vala,
            TokeiLanguageType::Verilog => LanguageType::Verilog,
            TokeiLanguageType::VerilogArgsFile => LanguageType::VerilogArgsFile,
            TokeiLanguageType::Vhdl => LanguageType::Vhdl,
            TokeiLanguageType::VimScript => LanguageType::VimScript,
            TokeiLanguageType::VisualBasic => LanguageType::VisualBasic,
            TokeiLanguageType::VisualStudioProject => LanguageType::VisualStudioProject,
            TokeiLanguageType::VisualStudioSolution => LanguageType::VisualStudioSolution,
            TokeiLanguageType::Vue => LanguageType::Vue,
            TokeiLanguageType::WebAssembly => LanguageType::WebAssembly,
            TokeiLanguageType::Wolfram => LanguageType::Wolfram,
            TokeiLanguageType::XSL => LanguageType::XSL,
            TokeiLanguageType::Xaml => LanguageType::Xaml,
            TokeiLanguageType::XcodeConfig => LanguageType::XcodeConfig,
            TokeiLanguageType::Xml => LanguageType::Xml,
            TokeiLanguageType::Xtend => LanguageType::Xtend,
            TokeiLanguageType::Yaml => LanguageType::Yaml,
            TokeiLanguageType::Zig => LanguageType::Zig,
            TokeiLanguageType::Zsh => LanguageType::Zsh,
        }
    }
}

#[derive(Debug, Clone, Default, juniper::GraphQLObject)]
pub struct LanguageMap {
    abap: Option<Language>,
    action_script: Option<Language>,
    ada: Option<Language>,
    agda: Option<Language>,
    alex: Option<Language>,
    asn1: Option<Language>,
    asp: Option<Language>,
    asp_net: Option<Language>,
    assembly: Option<Language>,
    auto_hot_key: Option<Language>,
    autoconf: Option<Language>,
    automake: Option<Language>,
    bash: Option<Language>,
    batch: Option<Language>,
    bright_script: Option<Language>,
    c: Option<Language>,
    c_header: Option<Language>,
    c_make: Option<Language>,
    c_sharp: Option<Language>,
    c_shell: Option<Language>,
    cabal: Option<Language>,
    cassius: Option<Language>,
    ceylon: Option<Language>,
    clojure: Option<Language>,
    clojure_c: Option<Language>,
    clojure_script: Option<Language>,
    cobol: Option<Language>,
    coffee_script: Option<Language>,
    cogent: Option<Language>,
    cold_fusion: Option<Language>,
    cold_fusion_script: Option<Language>,
    coq: Option<Language>,
    cpp: Option<Language>,
    cpp_header: Option<Language>,
    crystal: Option<Language>,
    css: Option<Language>,
    d: Option<Language>,
    dart: Option<Language>,
    device_tree: Option<Language>,
    dockerfile: Option<Language>,
    dot_net_resource: Option<Language>,
    dream_maker: Option<Language>,
    edn: Option<Language>,
    elisp: Option<Language>,
    elixir: Option<Language>,
    elm: Option<Language>,
    elvish: Option<Language>,
    emacs_dev_env: Option<Language>,
    erlang: Option<Language>,
    fen: Option<Language>,
    f_sharp: Option<Language>,
    fish: Option<Language>,
    forth: Option<Language>,
    fortran_legacy: Option<Language>,
    fortran_modern: Option<Language>,
    fstar: Option<Language>,
    gd_script: Option<Language>,
    glsl: Option<Language>,
    go: Option<Language>,
    graphql: Option<Language>,
    groovy: Option<Language>,
    hamlet: Option<Language>,
    handlebars: Option<Language>,
    happy: Option<Language>,
    haskell: Option<Language>,
    haxe: Option<Language>,
    hcl: Option<Language>,
    hex: Option<Language>,
    hlsl: Option<Language>,
    html: Option<Language>,
    idris: Option<Language>,
    ini: Option<Language>,
    intel_hex: Option<Language>,
    isabelle: Option<Language>,
    jai: Option<Language>,
    java: Option<Language>,
    java_script: Option<Language>,
    json: Option<Language>,
    jsx: Option<Language>,
    julia: Option<Language>,
    julius: Option<Language>,
    kakoune_script: Option<Language>,
    kotlin: Option<Language>,
    llvm: Option<Language>,
    lean: Option<Language>,
    less: Option<Language>,
    linker_script: Option<Language>,
    liquid: Option<Language>,
    lisp: Option<Language>,
    logtalk: Option<Language>,
    lua: Option<Language>,
    lucius: Option<Language>,
    madlang: Option<Language>,
    makefile: Option<Language>,
    markdown: Option<Language>,
    meson: Option<Language>,
    mint: Option<Language>,
    module_def: Option<Language>,
    ms_build: Option<Language>,
    mustache: Option<Language>,
    nim: Option<Language>,
    nix: Option<Language>,
    not_quite_perl: Option<Language>,
    o_caml: Option<Language>,
    objective_c: Option<Language>,
    objective_cpp: Option<Language>,
    org: Option<Language>,
    oz: Option<Language>,
    psl: Option<Language>,
    pascal: Option<Language>,
    perl: Option<Language>,
    perl6: Option<Language>,
    pest: Option<Language>,
    php: Option<Language>,
    polly: Option<Language>,
    pony: Option<Language>,
    post_css: Option<Language>,
    processing: Option<Language>,
    prolog: Option<Language>,
    protobuf: Option<Language>,
    pure_script: Option<Language>,
    python: Option<Language>,
    qcl: Option<Language>,
    qml: Option<Language>,
    r: Option<Language>,
    rpm_specfile: Option<Language>,
    racket: Option<Language>,
    rakefile: Option<Language>,
    razor: Option<Language>,
    re_structured_text: Option<Language>,
    ruby: Option<Language>,
    ruby_html: Option<Language>,
    rust: Option<Language>,
    s_recode: Option<Language>,
    sass: Option<Language>,
    scala: Option<Language>,
    scheme: Option<Language>,
    scons: Option<Language>,
    sh: Option<Language>,
    sml: Option<Language>,
    solidity: Option<Language>,
    specman_e: Option<Language>,
    spice: Option<Language>,
    sql: Option<Language>,
    stratego: Option<Language>,
    svg: Option<Language>,
    swift: Option<Language>,
    swig: Option<Language>,
    system_verilog: Option<Language>,
    tcl: Option<Language>,
    tex: Option<Language>,
    text: Option<Language>,
    toml: Option<Language>,
    twig: Option<Language>,
    type_script: Option<Language>,
    unreal_developer_markdown: Option<Language>,
    unreal_plugin: Option<Language>,
    unreal_project: Option<Language>,
    unreal_script: Option<Language>,
    unreal_shader: Option<Language>,
    unreal_shader_header: Option<Language>,
    ur_web: Option<Language>,
    ur_web_project: Option<Language>,
    vb6: Option<Language>,
    vb_script: Option<Language>,
    vala: Option<Language>,
    verilog: Option<Language>,
    verilog_args_file: Option<Language>,
    vhdl: Option<Language>,
    vim_script: Option<Language>,
    visual_basic: Option<Language>,
    visual_studio_project: Option<Language>,
    visual_studio_solution: Option<Language>,
    vue: Option<Language>,
    web_assembly: Option<Language>,
    wolfram: Option<Language>,
    xsl: Option<Language>,
    xaml: Option<Language>,
    xcode_config: Option<Language>,
    xml: Option<Language>,
    xtend: Option<Language>,
    yaml: Option<Language>,
    zig: Option<Language>,
    zsh: Option<Language>,
}

impl LanguageMap {
    pub fn add_language(&mut self, language: &Language) {
        match language.language_type {
            LanguageType::Abap => {
                self.abap = Some(language.clone());
            },
            LanguageType::ActionScript => {
                self.action_script = Some(language.clone());
            },
            LanguageType::Ada => {
                self.ada = Some(language.clone());
            },
            LanguageType::Agda => {
                self.agda = Some(language.clone());
            },
            LanguageType::Alex => {
                self.alex = Some(language.clone());
            },
            LanguageType::Asn1 => {
                self.asn1 = Some(language.clone());
            },
            LanguageType::Asp => {
                self.asp = Some(language.clone());
            },
            LanguageType::AspNet => {
                self.asp_net = Some(language.clone());
            },
            LanguageType::Assembly => {
                self.assembly = Some(language.clone());
            },
            LanguageType::AutoHotKey => {
                self.auto_hot_key = Some(language.clone());
            },
            LanguageType::Autoconf => {
                self.autoconf = Some(language.clone());
            },
            LanguageType::Automake => {
                self.automake = Some(language.clone());
            },
            LanguageType::Bash => {
                self.bash = Some(language.clone());
            },
            LanguageType::Batch => {
                self.batch = Some(language.clone());
            },
            LanguageType::BrightScript => {
                self.bright_script = Some(language.clone());
            },
            LanguageType::C => {
                self.c = Some(language.clone());
            },
            LanguageType::CHeader => {
                self.c_header = Some(language.clone());
            },
            LanguageType::CMake => {
                self.c_make = Some(language.clone());
            },
            LanguageType::CSharp => {
                self.c_sharp = Some(language.clone());
            },
            LanguageType::CShell => {
                self.c_shell = Some(language.clone());
            },
            LanguageType::Cabal => {
                self.cabal = Some(language.clone());
            },
            LanguageType::Cassius => {
                self.cassius = Some(language.clone());
            },
            LanguageType::Ceylon => {
                self.ceylon = Some(language.clone());
            },
            LanguageType::Clojure => {
                self.clojure = Some(language.clone());
            },
            LanguageType::ClojureC => {
                self.clojure_c = Some(language.clone());
            },
            LanguageType::ClojureScript => {
                self.clojure_script = Some(language.clone());
            },
            LanguageType::Cobol => {
                self.cobol = Some(language.clone());
            },
            LanguageType::CoffeeScript => {
                self.coffee_script = Some(language.clone());
            },
            LanguageType::Cogent => {
                self.cogent = Some(language.clone());
            },
            LanguageType::ColdFusion => {
                self.cold_fusion = Some(language.clone());
            },
            LanguageType::ColdFusionScript => {
                self.cold_fusion_script = Some(language.clone());
            },
            LanguageType::Coq => {
                self.coq = Some(language.clone());
            },
            LanguageType::Cpp => {
                self.cpp = Some(language.clone());
            },
            LanguageType::CppHeader => {
                self.cpp_header = Some(language.clone());
            },
            LanguageType::Crystal => {
                self.crystal = Some(language.clone());
            },
            LanguageType::Css => {
                self.css = Some(language.clone());
            },
            LanguageType::D => {
                self.d = Some(language.clone());
            },
            LanguageType::Dart => {
                self.dart = Some(language.clone());
            },
            LanguageType::DeviceTree => {
                self.device_tree = Some(language.clone());
            },
            LanguageType::Dockerfile => {
                self.dockerfile = Some(language.clone());
            },
            LanguageType::DotNetResource => {
                self.dot_net_resource = Some(language.clone());
            },
            LanguageType::DreamMaker => {
                self.dream_maker = Some(language.clone());
            },
            LanguageType::Edn => {
                self.edn = Some(language.clone());
            },
            LanguageType::Elisp => {
                self.elisp = Some(language.clone());
            },
            LanguageType::Elixir => {
                self.elixir = Some(language.clone());
            },
            LanguageType::Elm => {
                self.elm = Some(language.clone());
            },
            LanguageType::Elvish => {
                self.elvish = Some(language.clone());
            },
            LanguageType::EmacsDevEnv => {
                self.emacs_dev_env = Some(language.clone());
            },
            LanguageType::Erlang => {
                self.erlang = Some(language.clone());
            },
            LanguageType::FEN => {
                self.fen = Some(language.clone());
            },
            LanguageType::FSharp => {
                self.f_sharp = Some(language.clone());
            },
            LanguageType::Fish => {
                self.fish = Some(language.clone());
            },
            LanguageType::Forth => {
                self.forth = Some(language.clone());
            },
            LanguageType::FortranLegacy => {
                self.fortran_legacy = Some(language.clone());
            },
            LanguageType::FortranModern => {
                self.fortran_modern = Some(language.clone());
            },
            LanguageType::Fstar => {
                self.fstar = Some(language.clone());
            },
            LanguageType::GdScript => {
                self.gd_script = Some(language.clone());
            },
            LanguageType::Glsl => {
                self.glsl = Some(language.clone());
            },
            LanguageType::Go => {
                self.go = Some(language.clone());
            },
            LanguageType::Graphql => {
                self.graphql = Some(language.clone());
            },
            LanguageType::Groovy => {
                self.groovy = Some(language.clone());
            },
            LanguageType::Hamlet => {
                self.hamlet = Some(language.clone());
            },
            LanguageType::Handlebars => {
                self.handlebars = Some(language.clone());
            },
            LanguageType::Happy => {
                self.happy = Some(language.clone());
            },
            LanguageType::Haskell => {
                self.haskell = Some(language.clone());
            },
            LanguageType::Haxe => {
                self.haxe = Some(language.clone());
            },
            LanguageType::Hcl => {
                self.hcl = Some(language.clone());
            },
            LanguageType::Hex => {
                self.hex = Some(language.clone());
            },
            LanguageType::Hlsl => {
                self.hlsl = Some(language.clone());
            },
            LanguageType::Html => {
                self.html = Some(language.clone());
            },
            LanguageType::Idris => {
                self.idris = Some(language.clone());
            },
            LanguageType::Ini => {
                self.ini = Some(language.clone());
            },
            LanguageType::IntelHex => {
                self.intel_hex = Some(language.clone());
            },
            LanguageType::Isabelle => {
                self.isabelle = Some(language.clone());
            },
            LanguageType::Jai => {
                self.jai = Some(language.clone());
            },
            LanguageType::Java => {
                self.java = Some(language.clone());
            },
            LanguageType::JavaScript => {
                self.java_script = Some(language.clone());
            },
            LanguageType::Json => {
                self.json = Some(language.clone());
            },
            LanguageType::Jsx => {
                self.jsx = Some(language.clone());
            },
            LanguageType::Julia => {
                self.julia = Some(language.clone());
            },
            LanguageType::Julius => {
                self.julius = Some(language.clone());
            },
            LanguageType::KakouneScript => {
                self.kakoune_script = Some(language.clone());
            },
            LanguageType::Kotlin => {
                self.kotlin = Some(language.clone());
            },
            LanguageType::LLVM => {
                self.llvm = Some(language.clone());
            },
            LanguageType::Lean => {
                self.lean = Some(language.clone());
            },
            LanguageType::Less => {
                self.less = Some(language.clone());
            },
            LanguageType::LinkerScript => {
                self.linker_script = Some(language.clone());
            },
            LanguageType::Liquid => {
                self.liquid = Some(language.clone());
            },
            LanguageType::Lisp => {
                self.lisp = Some(language.clone());
            },
            LanguageType::Logtalk => {
                self.logtalk = Some(language.clone());
            },
            LanguageType::Lua => {
                self.lua = Some(language.clone());
            },
            LanguageType::Lucius => {
                self.lucius = Some(language.clone());
            },
            LanguageType::Madlang => {
                self.madlang = Some(language.clone());
            },
            LanguageType::Makefile => {
                self.makefile = Some(language.clone());
            },
            LanguageType::Markdown => {
                self.markdown = Some(language.clone());
            },
            LanguageType::Meson => {
                self.meson = Some(language.clone());
            },
            LanguageType::Mint => {
                self.mint = Some(language.clone());
            },
            LanguageType::ModuleDef => {
                self.module_def = Some(language.clone());
            },
            LanguageType::MsBuild => {
                self.ms_build = Some(language.clone());
            },
            LanguageType::Mustache => {
                self.mustache = Some(language.clone());
            },
            LanguageType::Nim => {
                self.nim = Some(language.clone());
            },
            LanguageType::Nix => {
                self.nix = Some(language.clone());
            },
            LanguageType::NotQuitePerl => {
                self.not_quite_perl = Some(language.clone());
            },
            LanguageType::OCaml => {
                self.o_caml = Some(language.clone());
            },
            LanguageType::ObjectiveC => {
                self.objective_c = Some(language.clone());
            },
            LanguageType::ObjectiveCpp => {
                self.objective_cpp = Some(language.clone());
            },
            LanguageType::Org => {
                self.org = Some(language.clone());
            },
            LanguageType::Oz => {
                self.oz = Some(language.clone());
            },
            LanguageType::PSL => {
                self.psl = Some(language.clone());
            },
            LanguageType::Pascal => {
                self.pascal = Some(language.clone());
            },
            LanguageType::Perl => {
                self.perl = Some(language.clone());
            },
            LanguageType::Perl6 => {
                self.perl6 = Some(language.clone());
            },
            LanguageType::Pest => {
                self.pest = Some(language.clone());
            },
            LanguageType::Php => {
                self.php = Some(language.clone());
            },
            LanguageType::Polly => {
                self.polly = Some(language.clone());
            },
            LanguageType::Pony => {
                self.pony = Some(language.clone());
            },
            LanguageType::PostCss => {
                self.post_css = Some(language.clone());
            },
            LanguageType::Processing => {
                self.processing = Some(language.clone());
            },
            LanguageType::Prolog => {
                self.prolog = Some(language.clone());
            },
            LanguageType::Protobuf => {
                self.protobuf = Some(language.clone());
            },
            LanguageType::PureScript => {
                self.pure_script = Some(language.clone());
            },
            LanguageType::Python => {
                self.python = Some(language.clone());
            },
            LanguageType::Qcl => {
                self.qcl = Some(language.clone());
            },
            LanguageType::Qml => {
                self.qml = Some(language.clone());
            },
            LanguageType::R => {
                self.r = Some(language.clone());
            },
            LanguageType::RPMSpecfile => {
                self.rpm_specfile = Some(language.clone());
            },
            LanguageType::Racket => {
                self.racket = Some(language.clone());
            },
            LanguageType::Rakefile => {
                self.rakefile = Some(language.clone());
            },
            LanguageType::Razor => {
                self.razor = Some(language.clone());
            },
            LanguageType::ReStructuredText => {
                self.re_structured_text = Some(language.clone());
            },
            LanguageType::Ruby => {
                self.ruby = Some(language.clone());
            },
            LanguageType::RubyHtml => {
                self.ruby_html = Some(language.clone());
            },
            LanguageType::Rust => {
                self.rust = Some(language.clone());
            },
            LanguageType::SRecode => {
                self.s_recode = Some(language.clone());
            },
            LanguageType::Sass => {
                self.sass = Some(language.clone());
            },
            LanguageType::Scala => {
                self.scala = Some(language.clone());
            },
            LanguageType::Scheme => {
                self.scheme = Some(language.clone());
            },
            LanguageType::Scons => {
                self.scons = Some(language.clone());
            },
            LanguageType::Sh => {
                self.sh = Some(language.clone());
            },
            LanguageType::Sml => {
                self.sml = Some(language.clone());
            },
            LanguageType::Solidity => {
                self.solidity = Some(language.clone());
            },
            LanguageType::SpecmanE => {
                self.specman_e = Some(language.clone());
            },
            LanguageType::Spice => {
                self.spice = Some(language.clone());
            },
            LanguageType::Sql => {
                self.sql = Some(language.clone());
            },
            LanguageType::Stratego => {
                self.stratego = Some(language.clone());
            },
            LanguageType::Svg => {
                self.svg = Some(language.clone());
            },
            LanguageType::Swift => {
                self.swift = Some(language.clone());
            },
            LanguageType::Swig => {
                self.swig = Some(language.clone());
            },
            LanguageType::SystemVerilog => {
                self.system_verilog = Some(language.clone());
            },
            LanguageType::Tcl => {
                self.tcl = Some(language.clone());
            },
            LanguageType::Tex => {
                self.tex = Some(language.clone());
            },
            LanguageType::Text => {
                self.text = Some(language.clone());
            },
            LanguageType::Toml => {
                self.toml = Some(language.clone());
            },
            LanguageType::Twig => {
                self.twig = Some(language.clone());
            },
            LanguageType::TypeScript => {
                self.type_script = Some(language.clone());
            },
            LanguageType::UnrealDeveloperMarkdown => {
                self.unreal_developer_markdown = Some(language.clone());
            },
            LanguageType::UnrealPlugin => {
                self.unreal_plugin = Some(language.clone());
            },
            LanguageType::UnrealProject => {
                self.unreal_project = Some(language.clone());
            },
            LanguageType::UnrealScript => {
                self.unreal_script = Some(language.clone());
            },
            LanguageType::UnrealShader => {
                self.unreal_shader = Some(language.clone());
            },
            LanguageType::UnrealShaderHeader => {
                self.unreal_shader_header = Some(language.clone());
            },
            LanguageType::UrWeb => {
                self.ur_web = Some(language.clone());
            },
            LanguageType::UrWebProject => {
                self.ur_web_project = Some(language.clone());
            },
            LanguageType::VB6 => {
                self.vb6 = Some(language.clone());
            },
            LanguageType::VBScript => {
                self.vb_script = Some(language.clone());
            },
            LanguageType::Vala => {
                self.vala = Some(language.clone());
            },
            LanguageType::Verilog => {
                self.verilog = Some(language.clone());
            },
            LanguageType::VerilogArgsFile => {
                self.verilog_args_file = Some(language.clone());
            },
            LanguageType::Vhdl => {
                self.vhdl = Some(language.clone());
            },
            LanguageType::VimScript => {
                self.vim_script = Some(language.clone());
            },
            LanguageType::VisualBasic => {
                self.visual_basic = Some(language.clone());
            },
            LanguageType::VisualStudioProject => {
                self.visual_studio_project = Some(language.clone());
            },
            LanguageType::VisualStudioSolution => {
                self.visual_studio_solution = Some(language.clone());
            },
            LanguageType::Vue => {
                self.vue = Some(language.clone());
            },
            LanguageType::WebAssembly => {
                self.web_assembly = Some(language.clone());
            },
            LanguageType::Wolfram => {
                self.wolfram = Some(language.clone());
            },
            LanguageType::XSL => {
                self.xsl = Some(language.clone());
            },
            LanguageType::Xaml => {
                self.xaml = Some(language.clone());
            },
            LanguageType::XcodeConfig => {
                self.xcode_config = Some(language.clone());
            },
            LanguageType::Xml => {
                self.xml = Some(language.clone());
            },
            LanguageType::Xtend => {
                self.xtend = Some(language.clone());
            },
            LanguageType::Yaml => {
                self.yaml = Some(language.clone());
            },
            LanguageType::Zig => {
                self.zig = Some(language.clone());
            },
            LanguageType::Zsh => {
                self.zsh = Some(language.clone());
            },
        }
    }
}

#[derive(Debug, Clone, juniper::GraphQLObject)]
pub struct Stats {
    pub blanks: i32,
    pub code: i32,
    pub comments: i32,
    pub lines: i32,
    pub name: String,
}

impl From<&TokeiStats> for Stats {
    fn from(tokei_stats: &TokeiStats) -> Self {
        Stats {
            blanks: tokei_stats.blanks as i32,
            code: tokei_stats.code as i32,
            comments: tokei_stats.comments as i32,
            lines: tokei_stats.lines as i32,
            name: tokei_stats.name.to_str().unwrap().to_string(),
        }
    }
}

#[derive(Debug, Clone, juniper::GraphQLObject)]
pub struct Language {
    pub language_type: LanguageType,
    pub blanks: i32,
    pub code: i32,
    pub comments: i32,
    pub lines: i32,
    pub stats: Vec<Stats>,
    pub inaccurate: bool,
}

impl From<&TokeiLanguage> for Language {
    fn from(tokei_language: &TokeiLanguage) -> Self {
        Language {
            language_type: LanguageType::D,
            blanks: tokei_language.blanks as i32,
            code: tokei_language.code as i32,
            comments: tokei_language.comments as i32,
            lines: tokei_language.lines as i32,
            stats: tokei_language.stats.iter().map(Stats::from).collect(),
            inaccurate: tokei_language.inaccurate,
        }
    }
}

#[derive(Debug, Clone, juniper::GraphQLObject)]
pub struct Languages {
    list: Vec<Language>,
    map: LanguageMap,
}

pub fn convert(tokei_languages: TokeiLanguages) -> Languages {
    let mut list = vec![];
    let mut map = LanguageMap::default();

    for (tokei_language_type, tokei_language) in tokei_languages.iter() {
        let language_type = LanguageType::from(tokei_language_type);
        let mut language = Language::from(tokei_language);

        language.language_type = language_type.clone();

        map.add_language(&language);
        list.push(language);
    }

    Languages {
        list,
        map,
    }
}

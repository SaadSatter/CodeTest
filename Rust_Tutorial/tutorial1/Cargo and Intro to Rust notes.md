Cargo: It is what is used to build and compile rust
When making a new project:
    In your terminal
        Create new project: cargo new <project_name>
    Creates
        a source folder
            Has a main file by default
        Cargo.toml : toms obvious minimal language
            Has the version
            Contains the dependencies
        .gitignore
        Cargo.lock : Keeps track of the versioning and dependencies
        target: Holds the executable file

Builing and Running using Cargo:
    First go into the project directory
    Then use cargo build
    Then cargo run

Running an executable:
    In your terminal:
        Go to /target/debug
        Then type ./<project_name>.exe
        OR
        In your project directory:
            type cargo run: this builds and compile and runs for you

Cargo Check: Checks whether the code can compile without compiling it  

Formatting your file automatically:
    Using rustfmt:
        In your source directory and terminal:
            rustfmt <src_file_name>

Rust Conventions to start:
    Note using rust, you should terminate the statements with semicolon
    You need a <main.rs> file and specifically a <fn main(){}> function
        Any other naming convention would cause an error

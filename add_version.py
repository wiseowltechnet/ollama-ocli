with open("src/main.rs", "r") as f:
    content = f.read()

old_struct = """    #[arg(short, long, default_value = "deepseek-coder:6.7b")]
    model: String,
    
    #[command(subcommand)]
    command: Option<Commands>,"""

new_struct = """    #[arg(short, long, default_value = "deepseek-coder:6.7b")]
    model: String,
    
    #[arg(short = 'V', long)]
    version: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,"""

content = content.replace(old_struct, new_struct)

old_main = """async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();"""

new_main = """async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.version {
        println!("🦉 OCLI v0.2.0");
        return Ok(());
    }
    
    let client = Client::new();"""

content = content.replace(old_main, new_main)

with open("src/main.rs", "w") as f:
    f.write(content)

print("✅ Added version flag")

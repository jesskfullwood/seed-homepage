# Convert Markdown files to HTML, then embed in Rust functions that return
# raw strings.
# We use this script and pandoc instead of built-in markdown functionality
# due to the need for syntax highlighting. Rust's Syntec library doesn't
# work on the wasm32-unknown-unknown target.

# Rererence https://pandoc.org/MANUAL.html

import os
import re

# ./pandoc --list-highlight-styles
# pygments tango espresso zenburn kate monochrome breezedark haddock
STYLE = "tango"
VERSION = "0.1.10"

def main():
    filenames = [
        "quickstart",
        "prereqs",
        "structure",
        "events",
        "components",
        "routing",
        "release_and_debugging",
        "element_deepdive",
        "misc",
        "about",
    ]
    
    for filename in filenames:
        # Perform the conversion
        os.system(f'.\pandoc .\markdown\{filename}.md -s --highlight-style {STYLE}\
        -o src/book/{filename}.html --metadata pagetitle="{filename}"')

        # Trim everything except for the HTML body; Pandoc outputs full files.
        with open(f'./src/book/{filename}.html', encoding="utf8") as f:
            data = f.read()

        # Correct a pandoc quirk.
        data = data.replace("’", "'")

        regex = re.compile(r'<body>(.*?)</body>', re.DOTALL)
        m = re.search(regex, data)

        body = m.groups(0)[0]

        body = re.sub(r'0\.1\.(d{1,2}?)', VERSION, body)

        # Create a new rust file
        with open(f'./src/book/{filename}.rs', 'w', encoding="utf8") as f:
            f.write('pub fn text() -> String {\n')
            f.write('r#"')
            f.write(body)
            f.write('"#.into()\n')
            f.write('}')

        # Clean up the temporary HTML files
        os.remove(f'./src/book/{filename}.html')

if __name__ == "__main__":
    main()
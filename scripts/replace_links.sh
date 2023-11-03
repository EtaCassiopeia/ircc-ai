#!/bin/bash

: '
Script Description:
This script recursively searches for Markdown (.md) files in a specified directory,
and updates the extensions of local internal links from .html or .htm to .md.
Additionally, it consolidates multiline links into single-line links to ensure
proper parsing.

How to Use:
1. Save this script to a file, e.g., replace_links.sh.
2. Make the script executable: chmod +x replace_links.sh.
3. Run the script, passing the directory containing the Markdown files as an argument:
   ./replace_links.sh /path/to/your/directory
'

# Check if a directory is provided
if [[ -z $1 ]]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Get the absolute path of the directory
dir=$(realpath $1)

# Define an awk script to handle multiline links
awk_script='
{
    line = $0
    while (match(line, /\]\([^)]*$/)) {
        getline next_line
        line = substr(line, 1, RSTART + RLENGTH - 1) next_line
    }
    print line
}'

# Find and process .md files
find "$dir" -type f -name "*.md" -exec sh -c "
    if [ -r '{}' ] && [ -w '{}' ]; then
        awk '$awk_script' '{}' > '{}'.temp && mv '{}'.temp '{}'
    else
        echo 'Skipping inaccessible file: {}'
    fi
" \;

# Now update the file extensions from .html or .htm to .md, excluding full URLs starting with http://www.canada.ca or https://www.canada.ca
find "$dir" -type f -name "*.md" -exec sh -c "
    if [ -r '{}' ] && [ -w '{}' ]; then
        perl -i.bak -pe 's|\\]\\((?!https?://www\\.canada\\.ca)([^)]+?)\\.html?\\)|](\$1.md)|g' '{}'
    else
        echo 'Skipping inaccessible file: {}'
    fi
" \;

# Optionally remove backup files (comment out the next line if you want to keep backup files)
find "$dir" -type f -name "*.md.bak" -exec rm {} +

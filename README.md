# note-manager-RUST ........................................
---

Created by: [Boahen Owusu Elijah Ewooral Dawson Kwame]
Email: [boahen.cos@gmail.com]
GitHub: [Ewooral]
* Project: CLI Tool for Note Management
  Project Description

    Create a CLI tool that allows users to manage notes. The tool will support the following features:

        * File Manipulation: Create, read, update, and delete note files.
        * Text Processing: Search for text within notes, and format text (e. g., convert markdown to plain text).
        * API Interaction: Sync notes with a remote server via a REST API.

* Features

    - Create a Note:
        Create a new note with a specified title and content.
        Save the note as a file in a designated directory.
    - Read a Note:
        Display the content of a note.
    - Update a Note:
        Edit the content of an existing note.
    - Delete a Note:
        Remove a note file.
    - Search Notes:
        Search for specific text within all notes.
    - Text Formatting:
        Convert markdown text to plain text or other formats.
    - Sync Notes:
        Fetch and push notes to a remote server using a REST API.



    
## Testing the CLI Application ...................................

1. Create a Note:

    sh > cargo run -- create "Meeting Notes" "Discuss project timeline and milestones."

- Expected Output:

    Created note: Meeting Notes

2. Read the Note:

    sh > cargo run -- read "Meeting Notes"

- Expected Output:

    Discuss project timeline and milestones.

3. Update the Note:

    sh > cargo run -- update "Meeting Notes" "Discuss project timeline and milestones. Review budget."

- Expected Output:

    Updated note: Meeting Notes

4. Delete the Note:

    sh > cargo run -- delete "Meeting Notes"

- Expected Output:

    Deleted note: Meeting Notes

5. Fetch Notes from API:

    sh > cargo run -- sync fetch --token YOUR_API_TOKEN

- Expected Output:

    Fetched notes with token: YOUR_API_TOKEN

6. Push Notes to API:

    sh > cargo run -- sync push --token YOUR_API_TOKEN

- Expected Output:

    Pushed notes with token: YOUR_API_TOKEN

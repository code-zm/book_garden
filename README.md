# Book Garden

A beautiful terminal-based reading tracker that grows a virtual plant as you progress through your books. Watch your garden flourish with each page you read!

## Features

- **Multi-Book Library**: Track unlimited books simultaneously
- **Reading Timer**: Accurately track time spent on each book
- **Progress Tracking**: Visual progress bars and page counters
- **Plant Visualization**: Watch a plant grow from seed to full bloom as you read
  - Stem grows with your progress
  - Leaves appear at milestones
  - Flower blooms when you finish the book
- **Statistics**: Total reading time and session count per book
- **Persistent Storage**: All progress automatically saved
- **Beautiful TUI**: Clean, intuitive terminal interface

## Installation

```bash
cargo build --release
```

The binary will be located at `target/release/book_garden`

## Usage

Run the program:
```bash
cargo run --release
# OR
./target/release/book_garden
```

### Book Selection Screen

When you first launch Book Garden, you'll see your library:

- **↑/↓**: Navigate between books
- **Enter**: Open selected book to start reading
- **N**: Add a new book (you'll be prompted for title and page count)
- **Q**: Quit

### Reading Screen

While reading a book:

- **Space**: Start/Pause the reading timer
- **S**: Stop the timer and save the session
- **←/→**: Adjust page count by 1
- **↑/↓**: Adjust page count by 10
- **B**: Return to book selection to switch books
- **Q**: Quit (automatically saves progress)

## How It Works

1. **Add a Book**: Press `N` in the library, enter the title and total pages
2. **Start Reading**: Select your book with `Enter`, press `Space` to start the timer
3. **Track Progress**: Update your current page using arrow keys as you read
4. **Watch Your Plant Grow**: The plant on the right side grows with your progress!
   - Sprouts immediately when you start
   - Leaves appear as you reach milestones
   - Flower blooms when you finish the book
5. **Switch Books**: Press `B` to return to your library and choose another book
6. **Everything Saves**: Progress, time, and sessions are automatically saved

## Data Storage

All data is stored in JSON format in the `reading_data/` directory:

- **library.json**: Contains all books and their progress
  - Book titles and page counts
  - Current page for each book
  - All reading sessions with timestamps
  - Total time spent per book

Each reading session records:
- Start and end pages
- Duration in seconds
- Timestamp

## The Garden Metaphor

Book Garden uses a growing plant to visualize your reading journey:

- **0-15%**: Stem begins to grow
- **15%+**: First leaves appear
- **25%+**: More leaves sprout
- **40%+**: Plant continues growing
- **55%+**: Additional leaves
- **70%+**: Nearly full growth
- **85%+**: Flower bud appears
- **100%**: Full bloom!

Start your reading garden today and watch your knowledge grow!

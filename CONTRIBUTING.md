## Contributing to 100-exercises-to-learn-rust 

We welcome contributions to the 100-exercises-to-learn-rust project! 
Whether you're fixing a bug, improving documentation, suggesting a new exercise, or adding tests, your help is valuable.

## Before You Start
Before contributing, please review the JetBrains Academy Educator Start Guide documentation: https://plugins.jetbrains.com/plugin/10081-jetbrains-academy/docs/educator-start-guide.html.

## How to Contribute

1. **Fork the Repository:** Start by forking this repository to your own GitHub account.
2. **Clone Your Fork:** Clone your forked repository to your local machine:
```git
git clone <YOUR_FORK_URL>
cd 100-exercises-to-learn-rust
```
3. **Create a New Branch:** Create a new branch for your changes. Use a descriptive name 
(e.g., `fix-typo-in-readme`, `add-ownership-exercise`, `improve-error-message`):
```git
git checkout -b your-branch-name
```

4. **Make Your Changes:** 
   - For existing exercises: Improve existing code, fix bugs, or enhance comments/documentation.
   - For new exercises:
     - Create a new Lesson in the appropriate Section, or create a new Section if not exist
     - Inside Lesson, create a Theory and Task part, following the structure and content style of existing lessons.
     - Clearly define the task using TODO comments and add placeholders.
     - Click on ***Preview Course*** button to preview the course

5. **Test Your Changes:** Before submitting, ensure your changes work as expected and don't introduce regressions.
   - Run all tests: `cargo test` 
   - If you added a new binary exercise, run it: `cargo run` (from its directory)
   - Ensure all existing tests still pass.
   
6. **Commit Your Changes:** Write clear, concise commit messages. A good commit message explains what you did and why. If your contribution addresses a specific exercise, please include the URL of that exercise from the `rust-exercises.com/100-exercises/` website in your commit message.
```git
git commit -m "fix: Correct typo in README's linter instructions"

# Example with exercise URL
git commit -m "feature: Added missing exercise Ticket V1/Ownership. https://rust-exercises.com/100-exercises/03_ticket_v1/06_ownership.html"

```
7. **Push to Your Fork:**
```git
git push origin your-branch-name
```

8. **Open a Pull Request (PR):** 
   - Go to the original repository on GitHub.
   - You should see a prompt to open a new Pull Request from your recently pushed branch.
   - Provide a detailed description of your changes in the PR. Reference any related issues.

## Code Style and Guidelines
All new contributions and code must align with the style and guidelines of the upstream repository: https://github.com/mainmatter/100-exercises-to-learn-rust.

Thank you for helping to improve this learning resource!
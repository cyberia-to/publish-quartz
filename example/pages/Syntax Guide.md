tags:: documentation, syntax, reference
alias:: Markdown Guide, Syntax Reference

- # Logseq Markdown Syntax Guide
- This page demonstrates all supported Logseq markdown features.
- ## Basic Text Formatting
	- **Bold text** using `**text**`
	- *Italic text* using `*text*`
	- ~~Strikethrough~~ using `~~text~~`
	- ^^Highlighted text^^ using `^^text^^`
	- `Inline code` using backticks
	- ==Marked text== using `==text==`
- ## Links
	- ### Wikilinks
		- Simple link: [[Getting Started]]
		- Link with alias: [[Getting Started|Home Page]]
		- Link to non-existent page: [[Future Page]]
	- ### External Links
		- [Logseq Website](https://logseq.com)
		- [GitHub](https://github.com/logseq/logseq)
	- ### Page References with Labels
		- Check out [our tasks]([[Tasks]]) for examples
- ## Properties
	- Properties are key-value pairs that add metadata to blocks.
	- status:: active
	- priority:: high
	- due-date:: 2024-02-01
	- category:: documentation
	- custom-property:: This is a custom value
- ## Block References
	- You can reference other blocks using their UUID:
	- Example reference: ((12345678-1234-1234-1234-123456789012))
- ## Code Blocks
	- ### JavaScript
	  ```javascript
	  function greet(name) {
	    console.log(`Hello, ${name}!`);
	  }
	  greet('World');
	  ```
	- ### Python
	  ```python
	  def fibonacci(n):
	      if n <= 1:
	          return n
	      return fibonacci(n-1) + fibonacci(n-2)

	  print(fibonacci(10))
	  ```
	- ### Rust
	  ```rust
	  fn main() {
	      println!("Hello from Rust!");
	  }
	  ```
- ## Lists
	- ### Unordered List
		- First item
		- Second item
			- Nested item
			- Another nested item
		- Third item
	- ### Numbered List
		- logseq.order-list-type:: number
		- First item
		- Second item
		- Third item
- ## Tables
	- | Feature | Supported | Notes |
	  |---------|-----------|-------|
	  | Wikilinks | Yes | Full support |
	  | Embeds | Yes | Page and block |
	  | Queries | Yes | Multiple types |
	  | Tasks | Yes | All markers |
- ## Blockquotes
	- > This is a blockquote.
	  > It can span multiple lines.
	- > [!tip] Callout Example
	  > This is a tip callout for important information.
	- > [!warning] Warning
	  > Be careful with this feature!
- ## Math (LaTeX)
	- Inline math: $E = mc^2$
	- Block math:
	  $$
	  \int_{a}^{b} f(x) \, dx = F(b) - F(a)
	  $$
- ## Horizontal Rule
	- ---
- ## Footnotes
	- Here's some text with a footnote[^1].
	- [^1]: This is the footnote content.
- ---
- Related pages: [[Getting Started]], [[Tasks]], [[Media and Embeds]]

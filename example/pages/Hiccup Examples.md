tags:: hiccup, edn, advanced

- # Hiccup Syntax in Logseq
- Logseq supports [Hiccup](https://github.com/weavejester/hiccup/wiki/Syntax) - a Clojure-style way to write HTML.
- ## Basic Text with Emphasis
	- [:p "Hello " [:em "World!"]]
- ## Code Display
	- [:div [:code "const x = 42"] " - JavaScript variable"]
- ## Links
	- [:a {:href "https://logseq.com"} "Visit Logseq"]
- ## Styled Elements
	- [:span {:style "color: #4cc38a; font-weight: bold;"} "Green bold text"]
- ## Lists
	- [:ul
	    [:li "First item"]
	    [:li "Second item"]
	    [:li "Third item"]]
- ## Combined Example
	- [:div
	    [:h3 "Note"]
	    [:p "This is a " [:strong "important"] " message."]
	    [:p [:em "Created with Hiccup syntax."]]]
- ## When to Use Hiccup
	- When markdown syntax is not enough
	- For custom styled elements
	- In advanced query results with `:view` function
	- In macros for dynamic content
- ---
- See also: [[Syntax Guide]]

tags:: edge-cases, special-characters

- # Dollar Sign Handling
- This page tests dollar sign escaping (important for LaTeX compatibility).
- ## Environment Variables
	- Common tokens like $HOME and $PATH need escaping
	- $TOKEN examples: $API_KEY, $SECRET
- ## Currency
	- The price is $100 USD
	- Budget: $50,000
- ## Math Mode
	- Inline math: $x^2 + y^2 = z^2$
	- Block math:
	  $$
	  f(x) = \sum_{i=0}^{n} \frac{a_i}{i!} x^i
	  $$
- ## Wikilinks with Dollar Signs
	- Link to [[$TOKEN Page]]
	- Reference: [[$VARIABLE Example]]
- ---
- This tests the dollar sign escaping logic in content.rs

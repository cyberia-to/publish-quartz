tags:: queries, advanced, demo

- # Logseq Queries Demo
- Queries allow you to dynamically pull content from your graph.
- ## Basic Queries
	- ### Page Tags Query
		- Find all pages tagged with "project":
		- {{query (page-tags [[project]])}}
	- ### Property Query
		- Find pages with status property:
		- {{query (property status)}}
- ## Task Queries
	- ### All TODO items
		- {{query (task TODO)}}
	- ### Active tasks (TODO or DOING)
		- {{query (task TODO DOING)}}
	- ### Completed tasks
		- {{query (task DONE)}}
- ## Priority Queries
	- ### High Priority `[#A]` Tasks
		- {{query (priority a)}}
	- ### Medium Priority `[#B]` Tasks
		- {{query (priority b)}}
- ## Date Range Queries
	- ### Journal entries from date range
		- {{query (between [[2024-01-01]] [[2024-01-15]])}}
- ## Table View
	- Use `query-properties::` to render results as a table:
	- query-properties:: [:page, :status, :tags]
	  query-sort-by:: name
	  {{query (page-tags [[project]])}}
- ## Sorted Queries
	- Sort by created date (descending):
	- query-sort-by:: created
	  query-sort-desc:: true
	  {{query (page-tags [[documentation]])}}
- ## Combined Queries
	- ### AND - Multiple conditions
		- {{query (and (page-tags [[project]]) (property status active))}}
	- ### OR - Either condition
		- {{query (or (page-tags [[project]]) (page-tags [[demo]]))}}
	- ### NOT - Exclude matches
		- Exclude pages named "Syntax Guide" (keeps pages that reference it):
		- {{query (and (page-tags [[documentation]]) (not (page [[Syntax Guide]])))}}
- ## Text Search
	- Search for specific text:
		- {{query "markdown"}}
- ## Notes
	- Queries are executed at build time by publish-quartz
	- Results are rendered as markdown lists or tables
	- Table view requires `query-properties::` option
	- Sorting available via `query-sort-by::` and `query-sort-desc::`
- ---
- See also: [[Syntax Guide]], [[Tasks]]

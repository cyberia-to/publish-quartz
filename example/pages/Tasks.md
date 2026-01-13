tags:: tasks, productivity, gtd

- # Task Management in Logseq
- Logseq supports various task markers and priorities for GTD-style task management.
- ## Task Markers
	- ### TODO - Not started
		- TODO Write documentation
		- TODO Review pull requests
		- TODO Update dependencies
	- ### DOING - In progress
		- DOING Implement new feature
		- DOING Debug authentication issue
	- ### NOW - Currently active
	  :LOGBOOK:
	  CLOCK: [2026-01-13 Tue 12:03:20]
	  :END:
		- NOW Working on example graph
	- ### LATER - Scheduled for later
		- LATER Research new technologies
		- LATER Plan Q2 roadmap
	- ### WAITING - Blocked/waiting
		- WAITING Waiting for API access
		- WAITING Pending review from team
	- ### DONE - Completed
		- DONE Set up project structure
		- DONE Configure CI/CD pipeline
		- DONE Write initial tests
	- ### CANCELLED - Not doing
		- CANCELLED Old feature request
- ## Priority Levels
	- [#A] High priority - Critical tasks
		- TODO [#A] Fix production bug
		- TODO [#A] Security update
	- [#B] Medium priority - Important but not urgent
		- TODO [#B] Improve performance
		- TODO [#B] Add logging
	- [#C] Low priority - Nice to have
		- TODO [#C] Refactor old code
		- TODO [#C] Update comments
- ## Scheduled Tasks
	- TODO Review weekly metrics
	  SCHEDULED: <2024-01-22 Mon>
	- TODO Monthly backup
	  SCHEDULED: <2024-02-01 Thu>
- ## Deadlines
	- TODO Submit report
	  DEADLINE: <2024-01-25 Thu>
	- TODO Complete milestone
	  DEADLINE: <2024-01-31 Wed>
- ## Combined Example
	- TODO [#A] Critical feature implementation
	  SCHEDULED: <2024-01-20 Sat>
	  DEADLINE: <2024-01-25 Thu>
	  assignee:: John
	  project:: Web App
	  estimate:: 4h
- ---
- See also: [[Getting Started]], [[Projects/Web App]]
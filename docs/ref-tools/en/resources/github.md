# GitHub

> Connect repos and control sync.

Manage your synced repos at [ref.tools/resources?tab=github](https://ref.tools/resources?tab=github)

## Which repos can I index?

* You may only index repos that you are a member of.
* If you would like a public repo indexed that is not already, [fill out the form here](https://tally.so/r/nrvBY2).

## What files are indexed?

* Small repos below 2,000 files index every file, including code files.
* Repos above 2,000 files index documentation files only.

## Synchronization

* Repos are synced on a 5 minute cron.
* Indexing is incremental so Ref will compare the current commit to the most recently indexed commit. This means if you modify the git history for your repo, indexing could fail and you will need to remove and re-add the repo from Ref.

## Authentication

* Ref uses personal access tokens to find and access your private repos.
* Adding a GitHub App for organizations is on the roadmap but not yet supported.


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt
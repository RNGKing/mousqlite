# mousqlite
Mousqlite is a self-hosted "distributed" sqlite solution that allows users to create, copy, sync and manage access to SQLite databases. 

# Expectations
* Mousqlite is not intended for production use as of this moment. Please read the license before you use this software. If you lose money, you should have read the license.
* This is a part time project for myself, if you end up needing fixes done PRs will be open.

# Features TODO
* Create SQLite Nano-Applications that serve as the IO to an sqlite database hosted on disk
* Create Process Manager that can be deployed into multiple nodes that serves as both a relay and access point to the front end
* Create a "Front End" web service that acts as an admin interface for user access as well as the "external" facing end points the system will use
* Creata a CLI application that communicates to the front end directly and allows authorized users to write direct queries to the database
* The goal is to migrate this system into a "wasi-compliant" system that can be orchestrated with a grouping of scripts.
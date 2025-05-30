[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/CyberCaban/board-app)
<p align="center">
    <h1 align="center">Web App</h1>
    <p align="center">
        Web file server with authentication. 
    </p>
</p>

## Made with

- Rust, a systems programming language.
- Diesel, a safe, extensible ORM and query builder.
- Rocket, a web framework for building web applications.
- PostgreSQL, a powerful, open source object-relational database system.
- Docker, a platform for containerizing and deploying applications.
- React, a JavaScript library for building user interfaces.
- Tailwind CSS, a utility-first CSS framework for building custom user interfaces.

## Features

- Authentication (username/password)
- File upload
- File download
- File deletion
- Realtime chat
- JWT token authentication
- Friend system with friend codes
- Public/private files separation (Anyone can see pulic files but only uploader can see private files)
- Docker containerization

## Usage

Prerequisites:

- [PostgreSQL](https://www.postgresql.org) (libpq-dev)
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/)
- [Docker](https://www.docker.com) (optional)

### With Docker

```bash
docker-compose up
```

You can now access the server at http://localhost:3000

### Without Docker

1. Create .env file with .env.local as example and put your database credentials there. It should look like this:

```env
PORT=8080 # required
ROCKET_ADDRESS=0.0.0.0 # required do not use 127.0.0.1
DATABASE_URL=postgres://username:password@localhost:5432/database_name # required
```

2. You should run the database server

3. Run database migrations

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

4. Build frontend inside the `www` folder:

```bash
cd www
npm install
npm run build
```

5. Start the server:

```bash
cargo run --release
```

You can now access the server at http://localhost:PORT where PORT is the port you specified in .env file.

## References

- [Docker](https://www.docker.com)
- [Rust](https://www.rust-lang.org)
- [Node.js](https://nodejs.org/en/)
- [PostgreSQL](https://www.postgresql.org)
- [Rocket](https://rocket.rs)
- [Diesel](https://diesel.rs)
- [Tailwind CSS](https://tailwindcss.com)
- [React](https://reactjs.org)
- [Vite](https://vitejs.dev)

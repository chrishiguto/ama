# AMA - Ask Me Anything!

https://github.com/user-attachments/assets/743c4b28-3dc6-481e-ad8b-b382712b6285

As part of my journey to learn Rust, I set out to build a web server that seamlessly integrates both API and WebSocket functionality. The goal was to dive deep into Rust's async ecosystem while creating something practical and interactive.

This project draws inspiration from a [Figma](<https://www.figma.com/design/Z636c5nyTsBAuDixtz5hvi/AMA---Ask-me-anything-(Community)?node-id=3-809&t=ffji0YwYrGG4rWMN-0>) design created by (Rocketseat)[https://www.rocketseat.com.br/]. Interestingly, I discovered later that they also had a crash course for implementing this in Golang. However, I wanted to make the journey my own by reimagining it in Rust—building everything from the ground up without external guides.

The result is a lightweight and minimalistic application where the primary focus is on exploring Rust's capabilities while delivering a sleek, user-friendly interface.

## Tech Stack

Rust is the star of the show here! This project was created as a way to practice the concepts I’ve been learning throughout my Rust journey—drawing inspiration from The Rust Programming Language book and various live streams I’ve watched over the past few months. While APIs may not be Rust’s most common use case, working in a familiar environment allowed me to solidify my understanding of the language. This approach was intentional, as it made the learning process more engaging and relevant to my background. It also provided the perfect opportunity to explore async Rust, which is a fascinating and unique aspect of the language!

Rust references:

- [Rust API template using Axum and MongoDB](https://github.com/ndelvalle/rustapi)
- [Actix Examples](https://github.com/actix/examples)
- [Rust Fullstack Actix Examples](https://github.com/steadylearner/Rust-Full-Stack/tree/master/actix/actix_examples)

MongoDB was a natural choice for this project due to its flexibility and developer-friendly features. Since this app revolves around handling dynamic "ask me anything" sessions, MongoDB's schema-less nature made it easier to adapt to varying data structures without compromising on speed or scalability. Additionally, its strong support for WebSocket-based real-time interactions ensured smooth integration with the Rust backend.

For the frontend, I chose React, paired with Vite and TailwindCSS to ensure a fast and modern development experience:

Vite provides an ultra-fast build and hot module replacement (HMR), making development swift and seamless. Its minimal configuration perfectly complements the simplicity I wanted for this project.
TailwindCSS allowed me to quickly prototype and style the application with utility-first CSS. Its customizable nature ensured that the UI aligned with the design while keeping the codebase clean and maintainable.
Combining these tools gave the frontend a polished and professional look while allowing me to focus on learning Rust for the backend.

There are some feature gaps—this project was meant to be a fast and focused experiment rather than a polished, production-ready application. Who knows, I might revisit and expand it in the future!

## Running the project

### 1. Clone the repository

Start by cloning the project to your local machine:

```bash
  git clone actix_examples
```

### 3. Create an environment file

Create an environment file name `.env` in the `server` folder with the following content:

```bash
  DATABASE_URL=mongodb://localhost:27017
  DATABASE_NAME=mongo
```

### 2. Set up a MongoDB instance

The project requires a MongoDB instance. You can choose any method that works best for you. If you prefer Docker (like I did), it’s as simple as running the following command:

```bash
  docker run -d -p 27017:27017 --name mongodb mongo
```

### 3. Run the backend

```bash
  cargo run
```

### 4. Run the frontend

```bash
  yarn dev
```

### 5. Open the application

Visit `http://localhost:5173` in you're browser, and you're good to go! :party:

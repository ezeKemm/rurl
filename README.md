<div align="center">
<pre>
██████══╗██╗   ██╗██████══╗██╗
██╔═══██║██║   ██║██╔═══██║██║
███████╔╝██║   ██║███████╔╝██║
██╔═══██╗██║   ██║██    ██╗██║
     ██║   ██║╚██████╔╝██    ██║███████╗
     ╚═╝   ╚═╝ ╚═════╝ ╚═    ╚═╝╚══════╝
     -----------------------------------
http request cli tool in rust

</pre>
<!-- [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) -->
</div> 

# rurl

rurl is like curl, but without all the complexity, usage features ... and sparkle. A barebones CLI tool for sending HTTP requests and receiving responses via JSON files. A handy, compact tool for testing or other purposes.

## Features

- **HTTP Methods:** Supports common HTTP methods such as GET, POST, PUT, DELETE, etc.
- **JSON Configuration:** Utilizes JSON files to define request parameters like URL, headers, body, etc.
- **Response Handling:** Stores the HTTP response, including status code, headers, and body in a JSON file.
<!-- - **Multiple Requests:** Allows executing multiple requests sequentially from a single configuration file. -->

## Getting Started

### Prerequisites

Make sure you have Rust installed on your system. If not, you can download it [here](https://www.rust-lang.org/).

### Installation

1. Clone the repository:

```bash
    git clone https://github.com/ezeKemm/rurl.git
```

2. Build the project:
```bash
    cd rurl
    cargo build --release
```

## Usage

The CLI tool uses a JSON configuration file to specify the details of the HTTP request.

Examples:

**A GET request** `get.json`
```json
{
   
"method": "GET",
"url": "https://api.example.com/users",
"headers": {
  "Authorization": "Bearer YOUR_ACCESS_TOKEN"
  },
}
```

**A POST request** `post.json`
```json
{
    "method": "POST",
    "url": "https://api.example.com/posts",
    "headers": {
      "Content-Type": "application/json"
    },
    "body": {
      "title": "New Post",
      "content": "This is the content of the new post."
    }
}
```

Save the JSON config file and run rurl in the command line

```bash
>> rurl post.json
```

## Options

`-h --help` : Display help information about rurl

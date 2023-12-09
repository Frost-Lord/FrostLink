# FrostLink

FrostLink is a reverse proxy written in Rust that supports routing requests from a local server to a specified domain. With FrostLink, you can easily turn a local server running on a specific port, such as `localhost:3000`, into a publicly accessible website connected to `example.com`.



## Features

- Supports HTTP and HTTPS(coming soon)
- Configurable through simple configuration files
- Efficient asynchronous handling of connections
- Logging of request paths, processing times, and client IP addresses

## How to Use

1. **Configure Domains:** Create configuration files in the `./domains` directory with the following format:
```
domain: example.com
ssl: true/false
location: localhost:3000
```
2. **Start the Reverse Proxy:** Run the main program to start the reverse proxy, and it will begin listening on ports 80 and 443 by default.

3. **Monitor Logs:** View the logs to see information about each request, including processing time, client IP address, domain, and request path.

## Example Configuration

You can configure multiple domains by creating `.conf` files inside the `./domains` directory. Here's an example:

### example.conf
```
domain: example.com
ssl: true
location: localhost:3000
```

## Contributing

Feel free to open issues or submit pull requests if you have ideas or encounter issues. Contributions are always welcome!

## License

FrostLink is open-source software, and its license information can be found in the LICENSE file in the repository.

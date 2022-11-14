# Wake On Lan server

A over-engineered solution to a minor problem.

Needs to have Rust and NodeJS installed.

## Docker

This project is available as a docker image, but in order to work we have to use
the Host network that docker provides.

```bash
docker run --rm -it --name wol-server --network host ghrc.io/jonavdm/wol-server
```

Note: because of docker this image can only run properly on a linux host. Not MacOS.

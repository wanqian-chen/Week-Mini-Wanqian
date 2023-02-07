## Week 3 Mini Project

### Steps to run

* `make format` to format the code
* `make lint` to lint
* `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
* `make deploy` which is this `cargo lambda deploy`

```Working Demo
(.venv) @wanqian-chen ➜ /workspaces/Week-Demo-Wanqian/marcopolo-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                marcopolo-lambda
{
  "msg": "Marco says Polo",
  "req_id": "5f33a044-cb3d-421e-846d-27033bef8d9d"
}
```
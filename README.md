# SVG to PNG Cloudflare Worker

SVG to PNG converter in Cloudflare Workers

# Installation

You need to setup [Rust](https://developers.cloudflare.com/workers/languages/rust/) for this after which you should be able to run `wrangler dev` and `wrangler deploy` without issue.

# Usage

https://svg-to-png.wilmake.workers.dev/{SVG-URL}

**Demo**: https://svg-to-png.wilmake.workers.dev/https://docs.tandoor.dev/logo_color.svg

Stresstest (100x)

```sh
bash -c 'for i in {1..100}; do curl -o /dev/null -s -w "%{time_total}\n" https://svg-to-png.wilmake.workers.dev/https://docs.tandoor.dev/logo_color.svg; done | sort -n | awk "BEGIN{print \"P80\tP95\tP99\"} {a[NR]=\$1} END{print a[int(NR*0.8)+1]\"\t\"a[int(NR*0.95)+1]\"\t\"a[int(NR*0.99)+1]}"'
```

Result:

```
P80	P95	P99
0.702912	0.803607	0.823520
```

For this one (2048x2048): https://raw.githubusercontent.com/janwilmake/svg-to-png-cf-worker/refs/heads/main/demo.svg

```sh
time (seq 1 100 | xargs -n1 -P100 -I{} bash -c 'start=$(date +%s%3N); curl -s -o /dev/null -w "%{time_total}\n" https://svg-to-png.wilmake.workers.dev/https://raw.githubusercontent.com/janwilmake/svg-to-png-cf-worker/refs/heads/main/demo.svg; end=$(date +%s%3N)') | awk '{sum+=$1; count++} END {print "Average latency:", sum/count, "seconds"}'
```

Result:

```
Average latency: 0.894171 seconds
```

### POST Request

You can also make a POST request with the SVG URL in the body. The body should be a JSON object containing the URL.

Example using `curl`:

```sh
curl -X POST -H "Content-Type: application/json" -d '{"url": "https://docs.tandoor.dev/logo_color.svg"}' https://svg-to-png.mrproper.dev
```

This will convert the SVG at the specified URL to a PNG and return the PNG image.

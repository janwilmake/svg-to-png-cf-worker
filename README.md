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

# Pricing

CPU Time
P50 225.05ms
P90 334.71ms
P99 418.73ms
P999 450.33ms

Cost per 1M requests: ~$6.30 [[1]](https://letmeprompt.com/rules-httpsuithu-xmkahs0)

# TODO

- See if https://github.com/thx/resvg-js may be a better alternative. may also work in a Cloudflare worker! https://github.com/thx/resvg-js/issues/343 https://x.com/duc__an/status/1958900650680721876
- Currently, text and images don't appear in the final png: https://svg-to-png.wilmake.workers.dev/https://sse.p0web.com/og.svg
- Try updating dependencies: https://letmeprompt.com/httpspastebincon-ajcjyo0
- the dream: scalable this: https://github.com/kane50613/takumi
- https://github.com/thx/resvg-js/issues/382

<!-- Try first:
```
curl -X POST 'https://external-svg-renderer.bannerify.co/api/svgstreaming/convert-to-png-stream' \
  --header 'Content-Type: application/json' \
  --data '{
    "svgContent": "<svg width=\"100\" height=\"100\"><circle cx=\"50\" cy=\"50\" r=\"40\" fill=\"red\"/></svg>",
    "scale": 1.0
  }' \
  --output out.png
``` -->

window.BENCHMARK_DATA = {
  "lastUpdate": 1671291173531,
  "repoUrl": "https://github.com/yewstack/yew",
  "entries": {
    "Yew master branch benchmarks (Lower is better)": [
      {
        "commit": {
          "author": {
            "name": "WorldSEnder",
            "username": "WorldSEnder",
            "email": "WorldSEnder@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "6160703c55e8bb13217879e4e05c014bac6abb69",
          "message": "Try fixing the post-comment benchmark action, take 2 (#3047)\n\n* Fix bash escaping\r\n* fix clippy issues\r\n* First checkout, then download artifacts",
          "timestamp": "2022-12-17T15:09:51Z",
          "url": "https://github.com/yewstack/yew/commit/6160703c55e8bb13217879e4e05c014bac6abb69"
        },
        "date": 1671291171623,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "251.9535",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "266.068",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "718.861",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "236.3025",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "169.4805",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "269.1675",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "3349.6935000000003",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "612.2175",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "231.9895",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.101119041442871",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.584461212158203",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.134069442749023",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.643394470214844",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.50889873504639",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1883.005",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "21.143999999999988",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "351.7578125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "241.1615",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "268.9095",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "624.1125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "112.509",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "133.2345",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "225.978",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "3156.0694999999996",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "610.4865",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "221.876",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.109342575073242",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.501262664794922",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.8795013427734375",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.27276611328125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.15388107299805",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1733.1799999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "13.307999999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.4521484375",
            "unit": ""
          }
        ]
      }
    ]
  }
}
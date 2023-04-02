window.BENCHMARK_DATA = {
  "lastUpdate": 1680465213367,
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
      },
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
          "id": "aebd22596360d5035e64e69a1c45c522588d1ce5",
          "message": "Reentrant event listeners (#3037)\n\n* add test case for reentrent event listeners\r\n* use Fn to allow reentrent event listeners",
          "timestamp": "2022-12-21T01:57:55Z",
          "url": "https://github.com/yewstack/yew/commit/aebd22596360d5035e64e69a1c45c522588d1ce5"
        },
        "date": 1671588776924,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "147.001",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "163.477",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "340.7155",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "137.88150000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "95.403",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "139.77800000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "1936.598",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "354.265",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "121.469",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1146507263183594",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.867193222045898",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.133938789367676",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.7159423828125",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.44162464141846",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1739.832",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "16.74799999999999",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.2353515625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "153.163",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "162.9765",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "285.71349999999995",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "50.531000000000006",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "76.16550000000001",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "117.197",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1846.531",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "338.5555",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "124.3445",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.1001243591308594",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.748593330383301",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.8921356201171875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.601091384887695",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.19051551818848",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1738.284",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "7.347999999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9189453125",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Jesper Olsen",
            "username": "jesper-olsen",
            "email": "43079279+jesper-olsen@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "d2f35e45b2f56faf7b55c0b45a0740cacee17a24",
          "message": "remove \"Next loop\" - introduction repeating in yew.rs/docs (#3040)",
          "timestamp": "2022-12-22T20:49:50Z",
          "url": "https://github.com/yewstack/yew/commit/d2f35e45b2f56faf7b55c0b45a0740cacee17a24"
        },
        "date": 1671742439022,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "148.5635",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "166.983",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "351.95550000000003",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "145.0365",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "99.3705",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "144.8955",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "1984.707",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "357.6115",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "126.0565",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.11507797241211",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.894545555114746",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.114010810852051",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.707507133483887",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.51797389984131",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1731.64",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "9.235999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.2353515625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "153.546",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "168.175",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "280.724",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "52.231",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "76.586",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "129.225",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1912.87",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "352.884",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "125.8645",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.034335136413574",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.7398834228515625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.891395568847656",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.179370880126953",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.1495885848999",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1739.104",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "11.383999999999997",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9189453125",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Simon",
            "username": "siku2",
            "email": "simon@siku2.io"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "7bfba6b31c843471cec4b92dbf54b795731dab95",
          "message": "Update SVG colors (#3064)",
          "timestamp": "2022-12-28T16:09:01Z",
          "url": "https://github.com/yewstack/yew/commit/7bfba6b31c843471cec4b92dbf54b795731dab95"
        },
        "date": 1672244857961,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "188.9685",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "198.353",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "580.7295",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "204.415",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "144.324",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "218.581",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2609.9845",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "459.464",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "170.84050000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "1.842369079589844",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.849325180053711",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "6.892726898193359",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.39849853515625",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.4372034072876",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1741.152",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "10.879999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.2275390625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "187.355",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "201.6865",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "466.5415",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "77.99199999999999",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "109.9",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "175.92700000000002",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2425.2415",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "442.7355",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "172.5615",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.114553451538086",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.470160484313965",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.556568145751953",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.519829750061035",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.20766639709473",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1733.288",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "35.87200000000002",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.92578125",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Muhammad Hamza",
            "username": "hamza1311",
            "email": "muhammadhamza1311@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "38e2478d9a2b7a251f6fd068675618326d18fd5a",
          "message": "Use SVG for logo on website (#3065)",
          "timestamp": "2022-12-30T12:37:10Z",
          "url": "https://github.com/yewstack/yew/commit/38e2478d9a2b7a251f6fd068675618326d18fd5a"
        },
        "date": 1672404827062,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "162.321",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "180.0535",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "440.18",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "178.608",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "123.976",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "186.538",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2216.9134999999997",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "383.792",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "144.54500000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.032376289367676",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.864995956420898",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.142358779907227",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.707266807556152",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.44162464141846",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1736.104",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "9.983999999999996",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "162.01749999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "178.8175",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "366.873",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "64.313",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "94.5555",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "165.3025",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2011.2315",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "373.0315",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "137.2255",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.1012725830078125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.7697343826293945",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.56385612487793",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.605169296264648",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.208330154418945",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1729.924",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "16.436",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Kai Salmon",
            "username": "kaisalmon",
            "email": "kaisalmon@hotmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "919695f5ef8df7a54d828295e58a85dbc5cdda52",
          "message": "The webworker in the web_worker_fib exampel is now marked as a relative path (#3057)",
          "timestamp": "2023-01-01T18:27:07Z",
          "url": "https://github.com/yewstack/yew/commit/919695f5ef8df7a54d828295e58a85dbc5cdda52"
        },
        "date": 1672598914594,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "207.5485",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "245.1795",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "845.6625",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "288.275",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "213.025",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "313.8595",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "3109.0015",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "579.113",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "233.5495",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.114421844482422",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.849248886108398",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.112492561340332",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.706183433532715",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.46574115753174",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1895.56",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "21.815999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "224.3",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "232.753",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "720.568",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "119.6325",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "156.37900000000002",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "263.702",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2869.8805",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "551.8455",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "229.2655",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.0710573196411133",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.470452308654785",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.6357316970825195",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.21102237701416",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.1596155166626",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1737.604",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "13.619999999999996",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "1d35148d3e95f7e5b5bcdd84e41c8148d38900cf",
          "message": "Bump prettier from 2.8.0 to 2.8.1 in /website (#3070)\n\nBumps [prettier](https://github.com/prettier/prettier) from 2.8.0 to 2.8.1.\r\n- [Release notes](https://github.com/prettier/prettier/releases)\r\n- [Changelog](https://github.com/prettier/prettier/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/prettier/prettier/compare/2.8.0...2.8.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: prettier\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-04T14:25:39Z",
          "url": "https://github.com/yewstack/yew/commit/1d35148d3e95f7e5b5bcdd84e41c8148d38900cf"
        },
        "date": 1672843225787,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "153.0405",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "166.5575",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "410.979",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "156.811",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "103.8285",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "157.4885",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "1988.0585",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "357.7515",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "129.2625",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1219377517700195",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.88664436340332",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.136319160461426",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.707278251647949",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.48378562927246",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1731.8159999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "10.528",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "151.93",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "168.7665",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "302.712",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "52.203",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "78.227",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "131.4615",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1875.495",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "355.645",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "130.3225",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.12338924407959",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.748276710510254",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.88995361328125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.523041725158691",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.18849849700928",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1736.756",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "8.423999999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "e65ee6059989d065004f951728cc0702f68716bb",
          "message": "Bump JamesSingleton/is-organization-member from 1.0.0 to 1.0.1 (#3075)\n\nBumps [JamesSingleton/is-organization-member](https://github.com/JamesSingleton/is-organization-member) from 1.0.0 to 1.0.1.\r\n- [Release notes](https://github.com/JamesSingleton/is-organization-member/releases)\r\n- [Commits](https://github.com/JamesSingleton/is-organization-member/compare/1.0.0...1.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: JamesSingleton/is-organization-member\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-04T14:43:22Z",
          "url": "https://github.com/yewstack/yew/commit/e65ee6059989d065004f951728cc0702f68716bb"
        },
        "date": 1672843650124,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "149.3785",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "165.487",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "373.1315",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "149.801",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "97.4025",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "146.815",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2011.7765",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "353.394",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "128.19150000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.110507011413574",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.872500419616699",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.112526893615723",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.771339416503906",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.49028778076172",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1881.01",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "7.672",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "150.2395",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "165.784",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "270.813",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "50.659",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "75.0625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "125.325",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1905.339",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "347.433",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "128.8",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.102935791015625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.761440277099609",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.882205009460449",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.456124305725098",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.13158702850342",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1881.13",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "8.155999999999995",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "e65ee6059989d065004f951728cc0702f68716bb",
          "message": "Bump JamesSingleton/is-organization-member from 1.0.0 to 1.0.1 (#3075)\n\nBumps [JamesSingleton/is-organization-member](https://github.com/JamesSingleton/is-organization-member) from 1.0.0 to 1.0.1.\r\n- [Release notes](https://github.com/JamesSingleton/is-organization-member/releases)\r\n- [Commits](https://github.com/JamesSingleton/is-organization-member/compare/1.0.0...1.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: JamesSingleton/is-organization-member\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-04T14:43:22Z",
          "url": "https://github.com/yewstack/yew/commit/e65ee6059989d065004f951728cc0702f68716bb"
        },
        "date": 1672844439583,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "154.898",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "172.943",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "405.611",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "167.6695",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "110.316",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "174.865",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2104.2015",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "385.1845",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "138.9515",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1145057678222656",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.867158889770508",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.136823654174805",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.769794464111328",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.51586055755615",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1879.45",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "8.279999999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "154.40449999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "172.144",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "337.7",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "61.056",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "91.531",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "153.369",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1919.8245",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "354.0135",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "133.0985",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.1002845764160156",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.740505218505859",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.8699798583984375",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.519089698791504",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.128920555114746",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1729.208",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "14.416",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "e65ee6059989d065004f951728cc0702f68716bb",
          "message": "Bump JamesSingleton/is-organization-member from 1.0.0 to 1.0.1 (#3075)\n\nBumps [JamesSingleton/is-organization-member](https://github.com/JamesSingleton/is-organization-member) from 1.0.0 to 1.0.1.\r\n- [Release notes](https://github.com/JamesSingleton/is-organization-member/releases)\r\n- [Commits](https://github.com/JamesSingleton/is-organization-member/compare/1.0.0...1.0.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: JamesSingleton/is-organization-member\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-01-04T14:43:22Z",
          "url": "https://github.com/yewstack/yew/commit/e65ee6059989d065004f951728cc0702f68716bb"
        },
        "date": 1672844513651,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "165.688",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "185.412",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "530.2080000000001",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "198.6835",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "135.8535",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "207.557",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2278.793",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "417.8255",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "158.346",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1003284454345703",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.866460800170898",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.137238502502441",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.651958465576172",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.51942729949951",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1731.448",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "17.511999999999986",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "163.0565",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "180.512",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "442.576",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "80.5475",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "109.883",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "179.595",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2095.4725",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "404.7905",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "155.31349999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "1.8639841079711912",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.524023056030273",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.657811164855957",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.5192461013793945",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.131402015686035",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1882.155",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "17.34399999999999",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cecile Tonglet",
            "username": "cecton",
            "email": "cecile.tonglet@cecton.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "d4c2f03c3d41a6ae3b3d80e3b91b509263cb68b1",
          "message": "Add method map() on Children to wrap easily (#3039)",
          "timestamp": "2023-01-05T10:30:05Z",
          "url": "https://github.com/yewstack/yew/commit/d4c2f03c3d41a6ae3b3d80e3b91b509263cb68b1"
        },
        "date": 1672915554842,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "150.845",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "169.50650000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "397.745",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "155.15300000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "103.3905",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "154.317",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2029.535",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "367.942",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "133.73950000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.10983943939209",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.873064994812012",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.103273391723633",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.674735069274902",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.44144916534424",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1880.625",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "12.871999999999993",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "352.232421875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "155.586",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "168.8065",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "320.04049999999995",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "54.0975",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "79.16900000000001",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "128.5045",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1934.077",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "352.9945",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "132.009",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.10208511352539",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.7631378173828125",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.861632347106934",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.548478126525879",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.16257572174072",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1739.1239999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "11.671999999999995",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.9228515625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Kaede Hoshikawa",
            "username": "futursolo",
            "email": "futursolo@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "b55be12d3c852a652d906b5883726f63dcc4f752",
          "message": "Prefer pop_first if it is available (#3084)",
          "timestamp": "2023-01-08T14:09:49Z",
          "url": "https://github.com/yewstack/yew/commit/b55be12d3c852a652d906b5883726f63dcc4f752"
        },
        "date": 1673188028029,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "163.582",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "178.5285",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "510.687",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "188.0385",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "127.219",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "182.8385",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2152.652",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "401.8995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "163.8",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1161346435546875",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.887014389038086",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.13503360748291",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.38072395324707",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.51946640014648",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1730.2600000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "16.175999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "351.2685546875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "159.781",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "174.81099999999998",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "406.3205",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "73.6665",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "95.885",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "165.439",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2139.916",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "384.5665",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "158.1895",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.114465713500977",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.760494232177734",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.564903259277344",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.277327537536621",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.16345405578613",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1882.8250000000005",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "11.691999999999997",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "346.9560546875",
            "unit": ""
          }
        ]
      },
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
          "id": "c5ffe601f2822c5e2a4fcd08aa502c7f63775354",
          "message": "Implement an internal DomSlot for positioning instead of NodeRef (#3048)\n\nuse instead of NodeRef, decoupling the two\r\nfixes #3043\r\n\r\n* implement internal DomSlot\r\n* move DomSlot into submodule of dom_bundle\r\n* hide behind feature csr\r\n* add test cases\r\n* write get in continuation style, this saves a clone\r\n* private DomSlot::get",
          "timestamp": "2023-01-09T14:08:08Z",
          "url": "https://github.com/yewstack/yew/commit/c5ffe601f2822c5e2a4fcd08aa502c7f63775354"
        },
        "date": 1673274460541,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "208.1795",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "219.22250000000005",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "473.761",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "204.3805",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "135.697",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "216.587",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "3046.877",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "565.8495",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "194.049",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.110912322998047",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.753366470336914",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "6.847302436828613",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.637653350830078",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "46.467485427856445",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1738",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "17.847999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "345.8447265625",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "202.5135",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "205.888",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "421.151",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "82.988",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "97.9215",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "138.9545",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "2705.5275",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "526.206",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "193.055",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.1113109588623047",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.6337175369262695",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.591825485229492",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.515214920043945",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "45.24872589111328",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1735.936",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "13.928",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "341.5341796875",
            "unit": ""
          }
        ]
      },
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
          "id": "c5ffe601f2822c5e2a4fcd08aa502c7f63775354",
          "message": "Implement an internal DomSlot for positioning instead of NodeRef (#3048)\n\nuse instead of NodeRef, decoupling the two\r\nfixes #3043\r\n\r\n* implement internal DomSlot\r\n* move DomSlot into submodule of dom_bundle\r\n* hide behind feature csr\r\n* add test cases\r\n* write get in continuation style, this saves a clone\r\n* private DomSlot::get",
          "timestamp": "2023-01-09T14:08:08Z",
          "url": "https://github.com/yewstack/yew/commit/c5ffe601f2822c5e2a4fcd08aa502c7f63775354"
        },
        "date": 1673276538720,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-vlatest-keyed 01_run1k",
            "value": "153.305",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 02_replace1k",
            "value": "165.8515",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 03_update10th1k_x16",
            "value": "384.8165",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 04_select1k",
            "value": "152.2945",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 05_swap1k",
            "value": "99.016",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 06_remove-one-1k",
            "value": "149.099",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 07_create10k",
            "value": "2014.3395",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 08_create1k-after1k_x2",
            "value": "357.7195",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 09_clear1k_x8",
            "value": "129.147",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 21_ready-memory",
            "value": "2.1094703674316406",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 22_run-memory",
            "value": "6.8716583251953125",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 23_update5-memory",
            "value": "7.111719131469727",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 25_run-clear-memory",
            "value": "5.6772613525390625",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 26_run-10k-memory",
            "value": "47.46957969665527",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 31_startup-ci",
            "value": "1878.4500000000005",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 33_startup-mainthreadcost",
            "value": "17.751999999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-vlatest-keyed 34_startup-totalbytes",
            "value": "351.763671875",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 01_run1k",
            "value": "153.974",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 02_replace1k",
            "value": "170.91699999999997",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 03_update10th1k_x16",
            "value": "324.975",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 04_select1k",
            "value": "56.2325",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 05_swap1k",
            "value": "79.065",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 06_remove-one-1k",
            "value": "141.028",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 07_create10k",
            "value": "1944.438",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 08_create1k-after1k_x2",
            "value": "347.6365",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 09_clear1k_x8",
            "value": "127.073",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 21_ready-memory",
            "value": "2.1019678115844727",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 22_run-memory",
            "value": "6.737038612365723",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 23_update5-memory",
            "value": "6.865349769592285",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 25_run-clear-memory",
            "value": "5.519906997680664",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 26_run-10k-memory",
            "value": "46.12140274047851",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 31_startup-ci",
            "value": "1729.888",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 32_startup-bt",
            "value": "0",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 33_startup-mainthreadcost",
            "value": "16.411999999999995",
            "unit": ""
          },
          {
            "name": "yew-vlatest-keyed 34_startup-totalbytes",
            "value": "347.451171875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Julius Lungys",
            "username": "voidpumpkin",
            "email": "32368314+voidpumpkin@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "48dd69cdc64ccb5ebc0eeac25aeeed1c21f950ac",
          "message": "Fix js-benchmark action (#3202)\n\n* temp\r\n\r\n* test2\r\n\r\n* test3\r\n\r\n* no ts\r\n\r\n* more\r\n\r\n* manifest\r\n\r\n* fix warning",
          "timestamp": "2023-04-01T21:23:53Z",
          "url": "https://github.com/yewstack/yew/commit/48dd69cdc64ccb5ebc0eeac25aeeed1c21f950ac"
        },
        "date": 1680385821763,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "160.8965",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "187.2715",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "556.5825",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "235.394",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "136.0435",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "181.011",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2262.897",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "424.569",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "159.74200000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.1154747009277344",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.849200248718262",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "6.829536437988281",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.643092155456543",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.82933235168457",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "2045.4159999999997",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "49.16",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "319.3120000000001",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9072265625",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "168.189",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "178.355",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "371.0935",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "72.2445",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "93.8735",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "142.8395",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2143.2495",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "410.6385",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "158.23",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "1.8704185485839844",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.476163864135742",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.783080101013184",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.4572954177856445",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.654747009277344",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1883.725",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "48.047999999999995",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "315.22400000000005",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.2373046875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "a85fcde100709ba3d09066e7d15b762f086a8c57",
          "message": "Bump lipsum from 0.8.2 to 0.9.0 (#3191)\n\nBumps [lipsum](https://github.com/mgeisler/lipsum) from 0.8.2 to 0.9.0.\r\n- [Release notes](https://github.com/mgeisler/lipsum/releases)\r\n- [Commits](https://github.com/mgeisler/lipsum/compare/0.8.2...0.9.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: lipsum\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-01T22:19:32Z",
          "url": "https://github.com/yewstack/yew/commit/a85fcde100709ba3d09066e7d15b762f086a8c57"
        },
        "date": 1680388005825,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "170.465",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "188.4515",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "603.983",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "249.8885",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "139.85500000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "221.878",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2310.9015",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "434.862",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "163.20049999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.102869033813477",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.837899208068848",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.095216751098633",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.649565696716309",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.82681846618652",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1916.79",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "53.57199999999999",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "321.872",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9072265625",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "166.529",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "187.8035",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "442.4715",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "91.723",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "108.436",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "179.0405",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2053.809",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "423.2225",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "160.2095",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.102825164794922",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.453996658325195",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.633249282836914",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.228420257568359",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.63933563232422",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1881.92",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "50.07199999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "324.9520000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.2373046875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "a85fcde100709ba3d09066e7d15b762f086a8c57",
          "message": "Bump lipsum from 0.8.2 to 0.9.0 (#3191)\n\nBumps [lipsum](https://github.com/mgeisler/lipsum) from 0.8.2 to 0.9.0.\r\n- [Release notes](https://github.com/mgeisler/lipsum/releases)\r\n- [Commits](https://github.com/mgeisler/lipsum/compare/0.8.2...0.9.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: lipsum\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-01T22:19:32Z",
          "url": "https://github.com/yewstack/yew/commit/a85fcde100709ba3d09066e7d15b762f086a8c57"
        },
        "date": 1680388755590,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "235.6445",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "274.29",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "694.7090000000001",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "317.033",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "161.84300000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "223.46499999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "3359.281",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "646.6455",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "243.2665",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.1036243438720703",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.641587257385254",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.095617294311523",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.6534223556518555",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.86964797973633",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1920.45",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "58.116",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "411.3280000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9091796875",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "241.092",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "263.1245",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "476.954",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "100.8305",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "111.315",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "182.1635",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "3190.9885",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "581.894",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "217.086",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.164029121398926",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.72719669342041",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.838287353515625",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.120563507080078",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.67897129058838",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "2047.57",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "56.824000000000005",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "360.1520000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.2373046875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "a85fcde100709ba3d09066e7d15b762f086a8c57",
          "message": "Bump lipsum from 0.8.2 to 0.9.0 (#3191)\n\nBumps [lipsum](https://github.com/mgeisler/lipsum) from 0.8.2 to 0.9.0.\r\n- [Release notes](https://github.com/mgeisler/lipsum/releases)\r\n- [Commits](https://github.com/mgeisler/lipsum/compare/0.8.2...0.9.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: lipsum\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-01T22:19:32Z",
          "url": "https://github.com/yewstack/yew/commit/a85fcde100709ba3d09066e7d15b762f086a8c57"
        },
        "date": 1680388814451,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "157.855",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "173.0125",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "432.769",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "220.7645",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "118.585",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "181.623",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2020.7935",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "376.0545",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "135.28050000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.101957321166992",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.850436210632324",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.090776443481445",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.579585075378418",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.858835220336914",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1882.495",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "51.744",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "323.7160000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9091796875",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "157.151",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "173.42399999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "315.92499999999995",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "65.80799999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "80.0445",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "145.0025",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "1913.279",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "368.458",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "132.0285",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.1231603622436523",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.724942207336426",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.7782392501831055",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.390934944152832",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.68040657043457",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1912.3350000000005",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "46.61600000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "322.3120000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.2373046875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "a85fcde100709ba3d09066e7d15b762f086a8c57",
          "message": "Bump lipsum from 0.8.2 to 0.9.0 (#3191)\n\nBumps [lipsum](https://github.com/mgeisler/lipsum) from 0.8.2 to 0.9.0.\r\n- [Release notes](https://github.com/mgeisler/lipsum/releases)\r\n- [Commits](https://github.com/mgeisler/lipsum/compare/0.8.2...0.9.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: lipsum\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-01T22:19:32Z",
          "url": "https://github.com/yewstack/yew/commit/a85fcde100709ba3d09066e7d15b762f086a8c57"
        },
        "date": 1680388967993,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "266.1",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "289.063",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "880.697",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "368.2725",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "216.36",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "315.0675",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "3603.777",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "680.8415",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "247.13",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "1.9126176834106443",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.847994804382324",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "6.807563781738281",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.6423139572143555",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.85669898986816",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1920.6849999999995",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "78.656",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "430.2960000000003",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9091796875",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "262.091",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "279.697",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "681.0275",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "124.449",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "159.05849999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "254.784",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "3571.5755",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "643.351",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "240.5005",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.164029121398926",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.515631675720215",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.84063720703125",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.453431129455566",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.680742263793945",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1907.58",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "63.58",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "459.46400000000034",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.2373046875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Martin Vilcans",
            "username": "vilcans",
            "email": "martin@librador.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "86b6cb4949e5e12e48ed6a79ff14e6d280f1b220",
          "message": "todomvc example: Disallow entering empty items (#3193)\n\nIt was possible to add an empty entry by typing whitespace into the\r\ntext field. This fixes that by trimming the string *before* we check\r\nwhether it's empty.",
          "timestamp": "2023-04-02T06:57:31Z",
          "url": "https://github.com/yewstack/yew/commit/86b6cb4949e5e12e48ed6a79ff14e6d280f1b220"
        },
        "date": 1680419745102,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "182.986",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "199.41",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "509.3085",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "250.545",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "133.06099999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "168.062",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2570.522",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "473.051",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "178.86849999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.164278984069824",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.640580177307129",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.092981338500977",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.580496788024902",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.82718849182129",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1916.935",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "50.53199999999999",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "311.06800000000015",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9306640625",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "190.041",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "192.406",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "354.356",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "74.702",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "91.1365",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "126.336",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2545.992",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "443.918",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "173.8225",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.1165637969970703",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.724053382873535",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.814769744873047",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.461942672729492",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.667503356933594",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1881.87",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "53.111999999999995",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "336.60400000000027",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.267578125",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Martin Vilcans",
            "username": "vilcans",
            "email": "martin@librador.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "86b6cb4949e5e12e48ed6a79ff14e6d280f1b220",
          "message": "todomvc example: Disallow entering empty items (#3193)\n\nIt was possible to add an empty entry by typing whitespace into the\r\ntext field. This fixes that by trimming the string *before* we check\r\nwhether it's empty.",
          "timestamp": "2023-04-02T06:57:31Z",
          "url": "https://github.com/yewstack/yew/commit/86b6cb4949e5e12e48ed6a79ff14e6d280f1b220"
        },
        "date": 1680419753932,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "169.6255",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "181.6925",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "548.567",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "264.317",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "141.91500000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "192.489",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2361.8779999999997",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "409.42",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "161.281",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.111639976501465",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.841168403625488",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.033093452453613",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.6441450119018555",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.86969757080078",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1882.35",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "50.33599999999999",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "328.52800000000025",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "302.9306640625",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "171.3465",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "189.291",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "380.489",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "76.477",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "100.813",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "147.04950000000002",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2190.139",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "397.1085",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "162.837",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.1640377044677734",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.662223815917969",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.842002868652344",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.51595401763916",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.68046760559082",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "2041.8979999999997",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "53.99999999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "389.28800000000024",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "299.267578125",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Kaede Hoshikawa",
            "username": "futursolo",
            "email": "futursolo@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "9d7fafa3ffab88d4c8fab0b5a0905cb396d1860f",
          "message": "Make VList's children Rc'ed (#3050)\n\n* Make VList's children Rc'ed.\r\n\r\n* Slightly optimise for bundle size.\r\n\r\n* Revert manual cloning.\r\n\r\n* Add a benchmark.\r\n\r\n* Revert \"Revert manual cloning.\"\r\n\r\nThis reverts commit 1f737f26f85a13e49186df71eb41425387f3aa02.\r\n\r\n* Revert \"Slightly optimise for bundle size.\"\r\n\r\nThis reverts commit 71cf2a1d36f1a99463c68aae3d128aa3be93dc03.\r\n\r\n* Add size marker.\r\n\r\n* Revert \"Add size marker.\"\r\n\r\nThis reverts commit 3ca55be4ea309f516d91ecd8e2aa8256dd810d11.\r\n\r\n* Update benchmark.\r\n\r\n* Fix children_mut visibility.\r\n\r\n* Try to exclude add_child behaviour.\r\n\r\n* Fix typo.\r\n\r\n* Adjust visibility and docs.\r\n\r\n* Fix hydration with empty children.",
          "timestamp": "2023-04-02T14:51:59Z",
          "url": "https://github.com/yewstack/yew/commit/9d7fafa3ffab88d4c8fab0b5a0905cb396d1860f"
        },
        "date": 1680448434668,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "232.1045",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "258.9035",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "713.8855000000001",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "327.525",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "170.83749999999998",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "259.4945",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "3373.428",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "616.717",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "215.5205",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.109281539916992",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.781790733337402",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "6.805858612060547",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.453805923461914",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.639360427856445",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1882.105",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "61.987999999999985",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "457.98",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "305.3486328125",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "228.312",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "250.963",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "504.8490000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "97.105",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "120.548",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "183.985",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2952.6645",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "622.143",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "218.182",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.110947608947754",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.430422782897949",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.841144561767578",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.5151872634887695",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.43055057525635",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1733.056",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "68.548",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "418.4279999999997",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "301.591796875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Kaede Hoshikawa",
            "username": "futursolo",
            "email": "futursolo@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "36aaecc430ca0b5ecaf3674ecd18f56d62f8acab",
          "message": "Update Dependencies with Breaking Changes (#3204)\n\n* Update axum.\r\n\r\n* Update base64.\r\n\r\n* Update NPM Dependencies.\r\n\r\n* Update Translations.\r\n\r\n* Make sure all docusaurus dependencies are with the same version.",
          "timestamp": "2023-04-02T15:16:33Z",
          "url": "https://github.com/yewstack/yew/commit/36aaecc430ca0b5ecaf3674ecd18f56d62f8acab"
        },
        "date": 1680449604884,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "153.5605",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "168.2015",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "424.8405",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "206.558",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "114.99",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "158.913",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2017.093",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "367.826",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "128.4785",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "1.8533973693847656",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.724618911743164",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "6.833595275878906",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.644451141357422",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.679443359375",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1880.64",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "50.348",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "290.404",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "305.3486328125",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "153.59",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "168.551",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "294.19",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "57.4315",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "76.78399999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "120.411",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "1895.2855",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "351.3805",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "128.19549999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.1010589599609375",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.685181617736816",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.842746734619141",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.494107246398926",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.38946914672851",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1883.575",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "52.32399999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "321.1520000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "301.591796875",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Greg Johnston",
            "username": "gbj",
            "email": "greg.johnston@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "bdf5712d969b27fdb122a5da0c5384f5a7d06f04",
          "message": "Incremental performance improvements to element creation (#3169)\n\n* enable interning\r\n\r\n* intern tag names\r\n\r\n* intern attribute keys and event listener types\r\n\r\n* intern attribute values\r\n\r\n* cache and clone elements\r\n\r\n* clean up the node cloning version a bit\r\n\r\n* use HashMap instead of Vec for element cache\r\n\r\n* Revert \"intern attribute values\"\r\n\r\nThis reverts commit 28653c4660dcf1942fab3b0ad7d4c840b96e0f2a.\r\n\r\n* add `enable-interning` feature to Yew that activates the same in wasm-bindgen\r\n\r\n* remove interning feature",
          "timestamp": "2023-04-02T19:29:21Z",
          "url": "https://github.com/yewstack/yew/commit/bdf5712d969b27fdb122a5da0c5384f5a7d06f04"
        },
        "date": 1680465173851,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "168.0695",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "174.74900000000002",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "519.0395",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "237.97",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "136.6145",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "181.0875",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "2104.803",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "390.5415",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "152.9065",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.1080589294433594",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.727455139160156",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.034625053405762",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.641870498657227",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.64723587036133",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1881.2",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "56.308",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "304.06400000000014",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "307.833984375",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "152.09199999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "170.00099999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "387.2845",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "70.4515",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "90.7885",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "142.02249999999998",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "2080.5735",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "365.818",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "150.832",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.1072864532470703",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.666869163513184",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.7856645584106445",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.518580436706543",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.39256858825683",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1921.28",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "55.32000000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "309.30400000000014",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "304.0791015625",
            "unit": ""
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Greg Johnston",
            "username": "gbj",
            "email": "greg.johnston@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "bdf5712d969b27fdb122a5da0c5384f5a7d06f04",
          "message": "Incremental performance improvements to element creation (#3169)\n\n* enable interning\r\n\r\n* intern tag names\r\n\r\n* intern attribute keys and event listener types\r\n\r\n* intern attribute values\r\n\r\n* cache and clone elements\r\n\r\n* clean up the node cloning version a bit\r\n\r\n* use HashMap instead of Vec for element cache\r\n\r\n* Revert \"intern attribute values\"\r\n\r\nThis reverts commit 28653c4660dcf1942fab3b0ad7d4c840b96e0f2a.\r\n\r\n* add `enable-interning` feature to Yew that activates the same in wasm-bindgen\r\n\r\n* remove interning feature",
          "timestamp": "2023-04-02T19:29:21Z",
          "url": "https://github.com/yewstack/yew/commit/bdf5712d969b27fdb122a5da0c5384f5a7d06f04"
        },
        "date": 1680465211596,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "yew-hooks-v0.19.3-keyed 01_run1k",
            "value": "258.77750000000003",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 02_replace1k",
            "value": "296.8905",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 03_update10th1k_x16",
            "value": "854.4315",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 04_select1k",
            "value": "356.48900000000003",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 05_swap1k",
            "value": "198.8225",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 06_remove-one-1k",
            "value": "289.7685",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 07_create10k",
            "value": "3497.4395",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 08_create1k-after1k_x2",
            "value": "656.235",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 09_clear1k_x8",
            "value": "245.6945",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 21_ready-memory",
            "value": "2.104269027709961",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 22_run-memory",
            "value": "6.760345458984375",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 23_update5-memory",
            "value": "7.039640426635742",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 25_run-clear-memory",
            "value": "5.705405235290527",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 26_run-10k-memory",
            "value": "46.67780590057373",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 31_startup-ci",
            "value": "1885.44",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 32_startup-bt",
            "value": "69.36000000000001",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 33_startup-mainthreadcost",
            "value": "483.24800000000016",
            "unit": ""
          },
          {
            "name": "yew-hooks-v0.19.3-keyed 34_startup-totalbytes",
            "value": "305.3486328125",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 01_run1k",
            "value": "252.1885",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 02_replace1k",
            "value": "275.7315",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 03_update10th1k_x16",
            "value": "605.7950000000001",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 04_select1k",
            "value": "109.1895",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 05_swap1k",
            "value": "122.988",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 06_remove-one-1k",
            "value": "208.3885",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 07_create10k",
            "value": "3306.0795",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 08_create1k-after1k_x2",
            "value": "620.2735",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 09_clear1k_x8",
            "value": "219.871",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 21_ready-memory",
            "value": "2.115320205688477",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 22_run-memory",
            "value": "6.65981388092041",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 23_update5-memory",
            "value": "6.842761993408203",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 25_run-clear-memory",
            "value": "5.342329025268555",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 26_run-10k-memory",
            "value": "45.593509674072266",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 31_startup-ci",
            "value": "1883.58",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 32_startup-bt",
            "value": "68.22399999999999",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 33_startup-mainthreadcost",
            "value": "432.75600000000014",
            "unit": ""
          },
          {
            "name": "yew-v0.20.0-keyed 34_startup-totalbytes",
            "value": "301.591796875",
            "unit": ""
          }
        ]
      }
    ]
  }
}
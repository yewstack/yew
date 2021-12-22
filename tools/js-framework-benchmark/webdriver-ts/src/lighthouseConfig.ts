export const lhConfig = {
  artifacts: {
    traces: {},
    devtoolsLogs: {}
  },

  audits: [
    "speed-index-metric",
    "estimated-input-latency",
    "first-meaningful-paint",
    "time-to-first-byte",
    "first-interactive",
    "consistently-interactive",
    "user-timings",
    "critical-request-chains",
    "byte-efficiency/total-byte-weight",
    "screenshot-thumbnails",
    "mainthread-work-breakdown",
    "bootup-time"
  ],

  categories: {
    performance: {
      name: "Performance",
      description: "These encapsulate your app's current performance and opportunities to improve it.",
      audits: [
        {id: "first-meaningful-paint", weight: 5},
        {id: "first-interactive", weight: 5},
        {id: "consistently-interactive", weight: 5},
        {id: "speed-index-metric", weight: 1},
        {id: "estimated-input-latency", weight: 1},
        {id: "time-to-first-byte", weight: 0},
        {id: "total-byte-weight", weight: 0},
        {id: "critical-request-chains", weight: 0},
        {id: "user-timings", weight: 0},
        {id: "bootup-time", weight: 0},
        {id: "screenshot-thumbnails", weight: 0},
        {id: "mainthread-work-breakdown", weight: 0}
      ]
    }
  }
};

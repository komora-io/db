# drama

Multi-threaded a/sync Scheduler that prioritizes harmonious execution
of multitenant heterogeneous workloads:

* async tasks
* blocking IO
* compute-heavy work

Anyone who has run high scale multitenant rust workloads has felt pain
when trying to achieve a reasonable latency/throughput position with
a reasonable hardware budget. When using executors that don't have any
notion of multi-tenant fairness or multi-resource scheduling efficiency,
it's not uncommon at all for people to try to statically partition certain
types of tasks across different executors or threadpools and end up with a
badly tuned, messy, bug-prone, expensive system that kind of lets you get
sleep most of the time. When using an executor that shares a static blocking
threadpool across all tasks and when many tasks try to use blocking threads on
that threadpool, it's not uncommon at all for mysterious deadlocks to start
happening in production. Nein danke!

#### priorities

* observability - operators should easily be able to see the scheduler's view of the system:
  * what are the heavy hitter tenants?
  * what tenants are associated with high queue depths?
  * what tenants get throttled the most while ensuring fairness for others?
  * what kinds of IO have various tenants used?
* testability - bugs should jump out on your laptop instead of in production
  * scheduler-aware fault injection for executing error handling code in your system
* protecting low-latency workloads from high throughput tasks with a fairness-aware scheduling algorithm
* prioritizing work that is likely to finish soon and release resources
  * writes > compute > reads > accepts (without causing starvation with hard priorities)
* IO queue depth awareness, only allowing spawns and accepts when the system is at a safe level of saturation
* elasticity - allowing individual tenants to consume unused resources when other tenants are idle
* a little harder to deadlock the entire system with cyclical dependencies on implicit blocking threadpools

#### non-priorities

* the simplest possible API

#### dedication

dedicated to the many burnouts induced and friendships destroyed by the drama surrounding async rust.

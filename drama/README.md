# drama

An a/sync scheduler that aims to keep larger scale distributed systems alive
in the presence of overload.

This isn't a general purpose async executor. It's specifically for systems that
will occasionally become overloaded while serving multitenant heterogenous workloads,
and need to gracefully degrade rather than completely lock up. It doesn't act like
an infinite queue. It is utilization and saturation-aware, and exerts backpressure.
It does not deadlock the entire system when bursts of tasks are spawned that may
perform blocking work by having implicit global dependencies on a shared static
blocking threadpool (side-eye intensifies).

#### key features: backpressure and multi-tenant fairness

* An executor that looks like an infinite queue will eventually oversaturate and destroy your latency.
* An executor that is effectively random or round robin in its work queue management is only
  appropriate for low intensity homogeneous workloads. When you are making a large scale system cost
  effective and trying to reduce server costs, you will sometimes be saturating resources
  and you need to stay available while doing so. The scheduler should not deadlock or cause
  unbounded resource usage in the presence of overload, as others do.
* It's a particular processes's job to reject work when it reaches saturation
  so that an upstream load balancer can actually do its job and balance the load.
  Ideally a service can understand its actual degree of saturation and communicate that
  early to a load balancing system to achieve better results with fewer explicit rejections.
* For most workloads, accepts cause reads, reads cause compute, compute causes writes.
* Writes tend to be associated with the end of resource lifetimes.
  They should be prioritized highly so that resources can be released soon after.
* When writes are not possible, compute will often lead to writes, so they should come next.
* Reads lead to more resources being consumed and should only be allowed when compute and
  writes are not already saturated (lining up to be serviced).
* Accepts lead to reads and should likewise only be serviced after the other classes of work.
* No class of work (accepts, reads, compute, writes) should be strictly prioritized over others,
  as many workloads deviate from the high-level general case, and priorities should actually
  be slightly weak to prevent starvation.
* The above prioritization should occur on a per-tenant basis, rather than process-wide.
  We want to prioritize newer work over older work in general to serve low latency workloads
  well.

#### priorities

* observability - operators should easily be able to see the scheduler's view of the system:
  * how utilized or saturated is the process overall?
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

Dedicated to async rust's many destroyed friendships and burnouts.
Hopefully this reduces the latter a little for those working on
tenant-dense large scale systems.

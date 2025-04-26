# drama

Async executor for massively multitenant workloads.

Intended for use in the komora db project and highly multitenant cloud workloads.

#### priorities

* building an actual scheduler
* protecting low-latency workloads from high throughput tasks
* prioritizing work that is likely to finish soon and release resources
  * writes > reads > accepts
* IO queue depth awareness, only allowing spawns and accepts when queues are acceptable
* elasticity - allowing individual tenants to consume unused resources when other tenants are idle

##### non-priorities

* extremely simple interface

##### dedication

dedicated to the hundreds (if not thousands) of friendships destroyed by the drama surrounding async rust.

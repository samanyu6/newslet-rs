# Lessons learnt

- Tests can be embedded or separated. Embedded tests have **full** access to code and is not ideal unless it's a lib or code that doesn't require security.
In our case, it's abstracted to a different dir, since we're building user facing APIs and we test *behaviour* and not functionality.
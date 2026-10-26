# Prior criteria - camera improvement
- Proposes storing the inverse of transform in a Camera field, computed outside ray_from_pixel.
- Updates the field where transform changes: new and with_transform.
- ray_from_pixel starts using the field; nothing deleted is still used.
- Does not change the result: the same rays for the same pixels.
- Does not claim it got faster; the gain is a hypothesis with a measurement plan.
- The change compiles and passes the renderer tests.
use crate::TenantId;

const N_BUCKETS: usize = 256 * 1024;

pub(crate) struct UtilizationSketch {
    buckets: Box<[u32; N_BUCKETS]>,
    total: u64,
    increments: u16,
}

impl Default for UtilizationSketch {
    fn default() -> UtilizationSketch {
        let vec = vec![0; N_BUCKETS];
        let bs = vec.into_boxed_slice();
        UtilizationSketch {
            buckets: bs.try_into().unwrap(),
            total: 0,
            increments: 0,
        }
    }
}

impl UtilizationSketch {
    pub fn update_and_get_relative_proportion(
        &mut self,
        tenant_id: TenantId,
        utilization: u16,
    ) -> f32 {
        self.total += utilization as u64;

        let bucket = tenant_id.id as usize % N_BUCKETS;
        self.buckets[bucket] += utilization as u32;

        self.increments += 1;

        if self.increments == u16::MAX {
            self.decrement();
        }

        self.buckets[bucket] as f32 / self.total.max(0) as f32
    }

    fn decrement(&mut self) {
        for slot in self.buckets.iter_mut() {
            *slot /= 2;
        }

        self.total /= 2;

        self.increments = 0;
    }
}

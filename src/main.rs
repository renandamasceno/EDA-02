use std::env;
use std::fs;
use std::process;

const MIN_HASH_CAPACITY: usize = 4;

struct Entry<V> {
    key: u32,
    value: V,
}

struct DynamicHashTable<V> {
    buckets: Vec<Vec<Entry<V>>>,
    len: usize,
}

impl<V> DynamicHashTable<V> {
    fn new() -> Self {
        Self {
            buckets: Self::empty_buckets(MIN_HASH_CAPACITY),
            len: 0,
        }
    }

    fn empty_buckets(capacity: usize) -> Vec<Vec<Entry<V>>> {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }
        buckets
    }

    fn capacity(&self) -> usize {
        self.buckets.len()
    }

    fn bucket_index(&self, key: u32) -> usize {
        Self::hash(key) & (self.capacity() - 1)
    }

    fn hash(key: u32) -> usize {
        let mut x = key as usize;
        x ^= x >> 16;
        x = x.wrapping_mul(0x7feb_352d);
        x ^= x >> 15;
        x = x.wrapping_mul(0x846c_a68b);
        x ^ (x >> 16)
    }

    fn get(&self, key: u32) -> Option<&V> {
        let index = self.bucket_index(key);
        self.buckets[index]
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| &entry.value)
    }

    fn get_mut(&mut self, key: u32) -> Option<&mut V> {
        let index = self.bucket_index(key);
        self.buckets[index]
            .iter_mut()
            .find(|entry| entry.key == key)
            .map(|entry| &mut entry.value)
    }

    fn contains_key(&self, key: u32) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: u32, value: V) -> Option<V> {
        let index = self.bucket_index(key);
        for entry in &mut self.buckets[index] {
            if entry.key == key {
                return Some(std::mem::replace(&mut entry.value, value));
            }
        }

        if (self.len + 1) * 4 >= self.capacity() * 3 {
            self.resize(self.capacity() * 2);
        }

        let index = self.bucket_index(key);
        self.buckets[index].push(Entry { key, value });
        self.len += 1;
        None
    }

    fn remove(&mut self, key: u32) -> Option<V> {
        let index = self.bucket_index(key);
        let bucket = &mut self.buckets[index];
        let position = bucket.iter().position(|entry| entry.key == key)?;
        let entry = bucket.swap_remove(position);
        self.len -= 1;

        if self.capacity() > MIN_HASH_CAPACITY && self.len * 4 <= self.capacity() {
            self.resize(self.capacity() / 2);
        }

        Some(entry.value)
    }

    fn keys_sorted(&self) -> Vec<u32> {
        let mut keys = Vec::with_capacity(self.len);
        for bucket in &self.buckets {
            for entry in bucket {
                keys.push(entry.key);
            }
        }
        keys.sort_unstable();
        keys
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_buckets = Self::empty_buckets(new_capacity.max(MIN_HASH_CAPACITY));
        for bucket in self.buckets.drain(..) {
            for entry in bucket {
                let index = Self::hash(entry.key) & (new_buckets.len() - 1);
                new_buckets[index].push(entry);
            }
        }
        self.buckets = new_buckets;
    }
}

struct VanEmdeBoas {
    bits: u8,
    min: Option<u32>,
    max: Option<u32>,
    summary: Option<Box<VanEmdeBoas>>,
    clusters: DynamicHashTable<VanEmdeBoas>,
}

impl VanEmdeBoas {
    fn new(bits: u8) -> Self {
        Self {
            bits,
            min: None,
            max: None,
            summary: None,
            clusters: DynamicHashTable::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.min.is_none()
    }

    fn lower_bits(&self) -> u8 {
        self.bits / 2
    }

    fn upper_bits(&self) -> u8 {
        self.bits - self.lower_bits()
    }

    fn high(&self, value: u32) -> u32 {
        value >> self.lower_bits()
    }

    fn low(&self, value: u32) -> u32 {
        value & ((1u32 << self.lower_bits()) - 1)
    }

    fn index(&self, high: u32, low: u32) -> u32 {
        (high << self.lower_bits()) | low
    }

    fn empty_insert(&mut self, value: u32) {
        self.min = Some(value);
        self.max = Some(value);
    }

    fn contains(&self, value: u32) -> bool {
        if self.min == Some(value) || self.max == Some(value) {
            return true;
        }

        if self.bits == 1 || self.is_empty() {
            return false;
        }

        let high = self.high(value);
        let low = self.low(value);
        self.clusters
            .get(high)
            .is_some_and(|cluster| cluster.contains(low))
    }

    fn insert(&mut self, value: u32) {
        if self.contains(value) {
            return;
        }
        self.insert_unique(value);
    }

    fn insert_unique(&mut self, mut value: u32) {
        if self.is_empty() {
            self.empty_insert(value);
            return;
        }

        if value < self.min.unwrap() {
            let old_min = self.min.unwrap();
            self.min = Some(value);
            value = old_min;
        }

        if self.bits > 1 {
            let high = self.high(value);
            let low = self.low(value);

            if !self.clusters.contains_key(high) {
                let upper_bits = self.upper_bits();
                self.clusters
                    .insert(high, VanEmdeBoas::new(self.lower_bits()));
                self.summary
                    .get_or_insert_with(|| Box::new(VanEmdeBoas::new(upper_bits)))
                    .insert_unique(high);
            }

            self.clusters
                .get_mut(high)
                .expect("cluster must exist after insertion")
                .insert_unique(low);
        }

        if value > self.max.unwrap() {
            self.max = Some(value);
        }
    }

    fn delete(&mut self, value: u32) {
        if self.contains(value) {
            self.delete_existing(value);
        }
    }

    fn delete_existing(&mut self, value: u32) {
        if self.min == self.max {
            self.min = None;
            self.max = None;
            return;
        }

        if self.bits == 1 {
            let remaining = if value == 0 { 1 } else { 0 };
            self.min = Some(remaining);
            self.max = Some(remaining);
            return;
        }

        let mut value_to_delete = value;
        if self.min == Some(value) {
            let first_cluster = self
                .summary
                .as_ref()
                .and_then(|summary| summary.min)
                .expect("non-empty vEB must have a non-empty summary");
            let first_low = self
                .clusters
                .get(first_cluster)
                .and_then(|cluster| cluster.min)
                .expect("summary points to a non-empty cluster");
            value_to_delete = self.index(first_cluster, first_low);
            self.min = Some(value_to_delete);
        }

        let high = self.high(value_to_delete);
        let low = self.low(value_to_delete);
        let cluster_max_after_delete;
        let cluster_is_empty;

        {
            let cluster = self
                .clusters
                .get_mut(high)
                .expect("value to delete must be in an existing cluster");
            cluster.delete_existing(low);
            cluster_max_after_delete = cluster.max;
            cluster_is_empty = cluster.is_empty();
        }

        if cluster_is_empty {
            self.clusters.remove(high);

            if let Some(summary) = self.summary.as_mut() {
                summary.delete_existing(high);
                if summary.is_empty() {
                    self.summary = None;
                }
            }
        }

        if self.max == Some(value_to_delete) {
            if cluster_is_empty {
                if let Some(summary) = self.summary.as_ref() {
                    let max_cluster = summary.max.expect("summary must have a maximum");
                    let max_low = self
                        .clusters
                        .get(max_cluster)
                        .and_then(|cluster| cluster.max)
                        .expect("summary maximum points to a non-empty cluster");
                    self.max = Some(self.index(max_cluster, max_low));
                } else {
                    self.max = self.min;
                }
            } else {
                self.max = Some(self.index(
                    high,
                    cluster_max_after_delete.expect("non-empty cluster must have a maximum"),
                ));
            }
        }
    }

    fn successor(&self, value: u32) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        if self.bits == 1 {
            return if value == 0 && self.max == Some(1) {
                Some(1)
            } else {
                None
            };
        }

        if value < self.min.unwrap() {
            return self.min;
        }

        let high = self.high(value);
        let low = self.low(value);

        if let Some(cluster) = self.clusters.get(high) {
            if cluster.max.is_some_and(|cluster_max| low < cluster_max) {
                let successor_low = cluster
                    .successor(low)
                    .expect("cluster maximum proves that a successor exists");
                return Some(self.index(high, successor_low));
            }
        }

        let successor_cluster = self
            .summary
            .as_ref()
            .and_then(|summary| summary.successor(high))?;
        let successor_low = self
            .clusters
            .get(successor_cluster)
            .and_then(|cluster| cluster.min)
            .expect("summary successor points to a non-empty cluster");
        Some(self.index(successor_cluster, successor_low))
    }

    fn predecessor(&self, value: u32) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        if self.bits == 1 {
            return if value == 1 && self.min == Some(0) {
                Some(0)
            } else {
                None
            };
        }

        if value > self.max.unwrap() {
            return self.max;
        }

        let high = self.high(value);
        let low = self.low(value);

        if let Some(cluster) = self.clusters.get(high) {
            if cluster.min.is_some_and(|cluster_min| low > cluster_min) {
                let predecessor_low = cluster
                    .predecessor(low)
                    .expect("cluster minimum proves that a predecessor exists");
                return Some(self.index(high, predecessor_low));
            }
        }

        if let Some(predecessor_cluster) = self
            .summary
            .as_ref()
            .and_then(|summary| summary.predecessor(high))
        {
            let predecessor_low = self
                .clusters
                .get(predecessor_cluster)
                .and_then(|cluster| cluster.max)
                .expect("summary predecessor points to a non-empty cluster");
            return Some(self.index(predecessor_cluster, predecessor_low));
        }

        if self.min.is_some_and(|minimum| value > minimum) {
            self.min
        } else {
            None
        }
    }

    fn elements_sorted(&self) -> Vec<u32> {
        let mut elements = Vec::new();

        if let Some(minimum) = self.min {
            elements.push(minimum);
        } else {
            return elements;
        }

        if self.bits == 1 {
            if self.max != self.min {
                elements.push(self.max.expect("non-empty vEB must have a maximum"));
            }
            return elements;
        }

        for high in self.clusters.keys_sorted() {
            let cluster = self
                .clusters
                .get(high)
                .expect("key collected from table must still exist");
            for low in cluster.elements_sorted() {
                elements.push(self.index(high, low));
            }
        }

        elements
    }

    fn first_level_string(&self) -> String {
        let Some(minimum) = self.min else {
            return String::new();
        };

        let mut parts = vec![format!("Min: {}", minimum)];
        for high in self.clusters.keys_sorted() {
            let cluster = self
                .clusters
                .get(high)
                .expect("key collected from table must still exist");
            let values = cluster
                .elements_sorted()
                .into_iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("C[{}]: {}", high, values));
        }

        parts.join(", ")
    }
}

fn parse_u32_word(text: &str, line_number: usize) -> Result<u32, String> {
    let value = text.parse::<u64>().map_err(|_| {
        format!(
            "linha {}: '{}' nao e um inteiro sem sinal valido",
            line_number, text
        )
    })?;

    if value > u32::MAX as u64 {
        return Err(format!(
            "linha {}: '{}' esta fora do intervalo de 32 bits sem sinal",
            line_number, text
        ));
    }

    Ok(value as u32)
}

fn run(input_path: &str) -> Result<(), String> {
    let content = fs::read_to_string(input_path)
        .map_err(|error| format!("nao foi possivel ler '{}': {}", input_path, error))?;
    let mut tree = VanEmdeBoas::new(32);

    for (line_index, raw_line) in content.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let operation = parts.next().expect("non-empty line has an operation");

        match operation {
            "INC" => {
                let value_text = parts
                    .next()
                    .ok_or_else(|| format!("linha {}: INC precisa de um valor", line_number))?;
                if parts.next().is_some() {
                    return Err(format!("linha {}: INC recebeu argumentos demais", line_number));
                }
                tree.insert(parse_u32_word(value_text, line_number)?);
            }
            "REM" => {
                let value_text = parts
                    .next()
                    .ok_or_else(|| format!("linha {}: REM precisa de um valor", line_number))?;
                if parts.next().is_some() {
                    return Err(format!("linha {}: REM recebeu argumentos demais", line_number));
                }
                tree.delete(parse_u32_word(value_text, line_number)?);
            }
            "SUC" => {
                let value_text = parts
                    .next()
                    .ok_or_else(|| format!("linha {}: SUC precisa de um valor", line_number))?;
                if parts.next().is_some() {
                    return Err(format!("linha {}: SUC recebeu argumentos demais", line_number));
                }
                let value = parse_u32_word(value_text, line_number)?;
                println!("SUC {}", value);
                match tree.successor(value) {
                    Some(successor) => println!("{}", successor),
                    None => println!("+INF"),
                }
            }
            "PRE" => {
                let value_text = parts
                    .next()
                    .ok_or_else(|| format!("linha {}: PRE precisa de um valor", line_number))?;
                if parts.next().is_some() {
                    return Err(format!("linha {}: PRE recebeu argumentos demais", line_number));
                }
                let value = parse_u32_word(value_text, line_number)?;
                println!("PRE {}", value);
                match tree.predecessor(value) {
                    Some(predecessor) => println!("{}", predecessor),
                    None => println!("-INF"),
                }
            }
            "IMP" => {
                if parts.next().is_some() {
                    return Err(format!("linha {}: IMP nao recebe argumentos", line_number));
                }
                println!("IMP");
                println!("{}", tree.first_level_string());
            }
            _ => {
                return Err(format!(
                    "linha {}: operacao desconhecida '{}'",
                    line_number, operation
                ));
            }
        }
    }

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("uso: {} <arquivo-de-entrada.txt>", args[0]);
        process::exit(1);
    }

    if let Err(error) = run(&args[1]) {
        eprintln!("erro: {}", error);
        process::exit(1);
    }
}

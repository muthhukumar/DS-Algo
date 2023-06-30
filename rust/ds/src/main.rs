use ds::queue::Queue;

fn main() {
    let mut queue: Queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    while let Some(e) = queue.dequeue() {
        println!("{}", e);
    }
}

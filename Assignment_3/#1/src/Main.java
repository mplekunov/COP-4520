import java.util.concurrent.ThreadLocalRandom;

import java.util.concurrent.locks.ReentrantReadWriteLock;

class ConcurrentLinkedList {
    private final ReentrantReadWriteLock lock = new ReentrantReadWriteLock();
    private Node head;

    public boolean contains(int tag) {
        lock.readLock().lock();
        try {
            Node current = head;
            while (current != null) {
                if (current.tag == tag) {
                    return true;
                }
                current = current.next;
            }
            return false;
        } finally {
            lock.readLock().unlock();
        }
    }

    public void insert(int tag) {
        lock.writeLock().lock();
        try {
            if (head == null || tag < head.tag) {
                head = new Node(tag, head);
                return;
            }

            Node current = head;
            while (current.next != null && tag > current.next.tag) {
                current = current.next;
            }
            current.next = new Node(tag, current.next);
        } finally {
            lock.writeLock().unlock();
        }
    }

    public void remove() {
        lock.writeLock().lock();
        try {
            if (head != null) {
                head = head.next;
            }
        } finally {
            lock.writeLock().unlock();
        }
    }

    private static class Node {
        final int tag;
        Node next;

        Node(int tag, Node next) {
            this.tag = tag;
            this.next = next;
        }
    }
}

class Servant implements Runnable {
    private final ConcurrentLinkedList list;
    private final int[] addPresents;

    Servant(ConcurrentLinkedList list, int[] addPresents) {
        this.list = list;
        this.addPresents = addPresents;
    }

    @Override
    public void run() {
        for (int i = 0; i < addPresents.length; i++) {
            list.insert(addPresents[i]);

            list.remove();
        }
    }
}

public class Main {
    public static void main(String[] args) throws InterruptedException {
        final int numServants = 4;
        final int numPresents = 500_000;

        ConcurrentLinkedList list = new ConcurrentLinkedList();
        int[][] addPresents = new int[numServants][numPresents / numServants];

        for (int i = 0; i < numServants; i++) {
            for (int j = 0; j < numPresents / numServants; j++) {
                addPresents[i][j] = i * numPresents / numServants + j;
            }
        }

        for (int i = 0; i < numServants; i++) {
            for (int j = addPresents[i].length - 1; j > 0; j--) {
                int index = ThreadLocalRandom.current().nextInt(j + 1);
                int temp = addPresents[i][index];
                addPresents[i][index] = addPresents[i][j];
                addPresents[i][j] = temp;
            }
        }

        long startTime = System.nanoTime();

        Thread[] servantThreads = new Thread[numServants];
        for (int i = 0; i < numServants; i++) {
            servantThreads[i] = new Thread(new Servant(list, addPresents[i]));
            servantThreads[i].start();
        }

        for (Thread servantThread : servantThreads) {
            servantThread.join();
        }

        long endTime = System.nanoTime();

        long duration = (endTime - startTime) / 1_000_000;

        System.out.println("All servants have finished writing cards.");
        System.out.println("Execution time: " + duration + " ms");
    }
}

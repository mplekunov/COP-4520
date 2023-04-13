import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentSkipListSet;
import java.util.Random;
import java.time.LocalDateTime;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;

class SharedTemperatureData {
    private final ConcurrentHashMap<Integer, ConcurrentHashMap<LocalDateTime, ConcurrentSkipListSet<Integer>>> sensorData;

    public SharedTemperatureData() {
        this.sensorData = new ConcurrentHashMap<>();
        for (int i = 0; i < 8; i++) {
            this.sensorData.put(i, new ConcurrentHashMap<>());
        }
    }

    public void addTemperatureReading(int sensorId, int temperature) {
        LocalDateTime currentHour = LocalDateTime.now().truncatedTo(ChronoUnit.HOURS);
        sensorData.get(sensorId).computeIfAbsent(currentHour, k -> new ConcurrentSkipListSet<>()).add(temperature);
    }

    public ConcurrentHashMap<Integer, ConcurrentHashMap<LocalDateTime, ConcurrentSkipListSet<Integer>>> getSensorData() {
        return sensorData;
    }
}

class TemperatureSensor implements Runnable {
    private final int sensorId;
    private final SharedTemperatureData sharedData;
    private final Random random;

    public TemperatureSensor(int sensorId, SharedTemperatureData sharedData) {
        this.sensorId = sensorId;
        this.sharedData = sharedData;
        this.random = new Random();
    }

    @Override
    public void run() {
        while (true) {
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }

            int temperature = random.nextInt(171) - 100;
            sharedData.addTemperatureReading(sensorId, temperature);
        }
    }
}


class TemperatureReportGenerator implements Runnable {
    private final SharedTemperatureData sharedData;

    public TemperatureReportGenerator(SharedTemperatureData sharedData) {
        this.sharedData = sharedData;
    }

    @Override
    public void run() {
        while (true) {
            try {
                Thread.sleep(15 * 1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            
            List<Integer> top5Highest = getTop5HighestTemperatures();
            List<Integer> top5Lowest = getTop5LowestTemperatures();
            int[] largestDifferenceInterval = getLargestTemperatureDifferenceInterval();

            System.out.println();
            System.out.println("Hourly report:");
            System.out.println("Top 5 highest temperatures: " + top5Highest);
            System.out.println("Top 5 lowest temperatures: " + top5Lowest);
            System.out.println("Largest temperature difference interval: " +
                    largestDifferenceInterval[0] + " to " + largestDifferenceInterval[1]);
        }
    }

    private List<Integer> getTop5HighestTemperatures() {
        LocalDateTime previousHour = LocalDateTime.now().minusSeconds(1).truncatedTo(ChronoUnit.HOURS);
        return sharedData.getSensorData().values().stream()
                .flatMap(map -> map.getOrDefault(previousHour, new ConcurrentSkipListSet<>()).stream())
                .sorted(Comparator.reverseOrder())
                .limit(5)
                .collect(Collectors.toList());
    }
    
    private List<Integer> getTop5LowestTemperatures() {
        LocalDateTime previousHour = LocalDateTime.now().minusSeconds(1).truncatedTo(ChronoUnit.HOURS);
        return sharedData.getSensorData().values().stream()
                .flatMap(map -> map.getOrDefault(previousHour, new ConcurrentSkipListSet<>()).stream())
                .sorted()
                .limit(5)
                .collect(Collectors.toList());
    }    

    private int[] getLargestTemperatureDifferenceInterval() {
        LocalDateTime previousHour = LocalDateTime.now().minusSeconds(10).truncatedTo(ChronoUnit.HOURS);
        int[] interval = new int[]{0, 10};
        int maxDifference = 0;
    
        for (int i = 0; i <= 50; i++) {
            int finalI = i;
            List<Integer> temperaturesInInterval = sharedData.getSensorData().values().stream()
                    .flatMap(map -> map.getOrDefault(previousHour, new ConcurrentSkipListSet<>()).stream())
                    .filter(temp -> temp >= finalI && temp < finalI + 10)
                    .collect(Collectors.toList());
    
            if (!temperaturesInInterval.isEmpty()) {
                int highest = temperaturesInInterval.stream()
                        .max(Comparator.naturalOrder())
                        .orElse(Integer.MIN_VALUE);
    
                int lowest = temperaturesInInterval.stream()
                        .min(Comparator.naturalOrder())
                        .orElse(Integer.MAX_VALUE);
    
                int difference = highest - lowest;
    
                if (difference > maxDifference) {
                    maxDifference = difference;
                    interval[0] = i;
                    interval[1] = i + 10;
                }
            }
        }
    
        return interval;
    }   
}

public class Main {
    public static void main(String[] args) {
        SharedTemperatureData sharedData = new SharedTemperatureData();
        List<Thread> sensorThreads = new ArrayList<>();

        for (int i = 0; i < 8; i++) {
            sensorThreads.add(new Thread(new TemperatureSensor(i, sharedData)));
        }

        Thread reportGeneratorThread = new Thread(new TemperatureReportGenerator(sharedData));

        sensorThreads.forEach(Thread::start);
        reportGeneratorThread.start();

        try {
            reportGeneratorThread.join();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }
}
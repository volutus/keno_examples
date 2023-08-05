import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Runner
{
	private static final int ITERATIONS = 10000000;

	private static List<Integer> ALL_NUMBERS = IntStream.range(0, 81).boxed().collect(Collectors.toList());
	private static Map<Integer, List<Integer>> payouts = new HashMap<>();

	public static void main(String[] args)
	{
		long start = System.nanoTime();
		init();
		List<Integer> spots = IntStream.range(1, 11).boxed().collect(Collectors.toList());

		spots.parallelStream().forEach(o -> calculate(o));
//		for (Integer spot: spots)
//		{
//			calculate(spot);
//		}
		long end = System.nanoTime();
		long run = (end - start) / 1000000L;
		System.out.println("Finished in " + run + " milliseconds");

	}

	private static void init()
	{
		payouts.put(1, List.of(0, 2));
		payouts.put(2, List.of(0, 0, 10));
		payouts.put(3, List.of(0, 0, 2, 23));
		payouts.put(4, List.of(0, 0, 1, 5, 55));
		payouts.put(5, List.of(0, 0, 0, 2, 20, 300));
		payouts.put(6, List.of(0, 0, 0, 1, 6, 55, 1000));
		payouts.put(7, List.of(1, 0, 0, 0, 2, 20, 100, 5000));
		payouts.put(8, List.of(2, 0, 0, 0, 0, 6, 75, 550, 10000));
		payouts.put(9, List.of(2, 0, 0, 0, 0, 5, 20, 125, 3000, 30000));
		payouts.put(10, List.of(5, 0, 0, 0, 0, 2, 10, 45, 300, 5000, 100000));
	}

	private static void calculate(int spot)
	{
		long startingBalance = ITERATIONS;
		long currentBalance = startingBalance;
		for (int i = 0; i < ITERATIONS; i++)
		{
			currentBalance--;
			long winnings = play(spot);
			currentBalance += winnings;
		}
		long gainLoss = currentBalance - startingBalance;

		String outputString = """
Results for %d-spot: 
Starting Balance: %d
Current Balance: %d
GAIN/LOSS: %d
""";
		String output = String.format(outputString, spot, startingBalance, currentBalance, gainLoss);
		System.out.println(output);
	}

	private static long play(int spots)
	{
		Random random = new Random();
		Set<Integer> playerNumbers = new LinkedHashSet<>();
		while (playerNumbers.size() < spots)
		{
			int randomSpot = random.nextInt(80) + 1;
			playerNumbers.add(randomSpot);
		}

		Set<Integer> winningNumbers = new LinkedHashSet<>();
		while (winningNumbers.size() < 20)
		{
			int randomSpot = random.nextInt(80) + 1;
			winningNumbers.add(randomSpot);
		}

		int matches = 0;
		for (int spot: playerNumbers)
		{
			if (winningNumbers.contains(spot))
			{
				matches++;
			}
		}

		long winnings = payouts.get(spots).get(matches);
		return winnings;
	}
}

# Assignment 2
Language used for this Assignment is Rust.

### How to run
Simply use `cargo run --release` command to run the code in the "release" mod. Release mod provides optimizations and therefore, code runs faster.

### About

#1 Minotaur’s Birthday Party

The Minotaur invited N guests to his birthday party. When the guests arrived, he made the following announcement. 
The guests may enter his labyrinth, one at a time and only when he invites them to do so. At the end of the labyrinth, the Minotaur placed a birthday cupcake on a plate. 
When a guest finds a way out of the labyrinth, he or she may decide to eat the birthday cupcake or leave it. If the cupcake is eaten by the previous guest, the next guest will find 
the cupcake plate empty and may request another cupcake by asking the Minotaur’s servants. When the servants bring a new cupcake the guest may decide to eat it or leave 
it on the plate. The Minotaur’s only request for each guest is to not talk to the other guests about her or his visit to the labyrinth after the game has started. The guests are allowed to come up 
with a strategy prior to the beginning of the game. There are many birthday cupcakes, so the Minotaur may pick the same guests multiple times and ask them to enter the 
labyrinth. Before the party is over, the Minotaur wants to know if all of his guests have had the chance to enter his labyrinth. To do so, the guests must announce that they have 
all visited the labyrinth at least once. Now the guests must come up with a strategy to let the Minotaur know that every guest entered the Minotaur’s labyrinth. 

It is known that there is already a birthday cupcake left at the labyrinth’s exit at the start of the game. 

How would the guests do this and not disappoint his generous and a bit temperamental host? 

Solution:

So, we need to be sure that each guest visited labyrinth at least once. We know that guests cannot interact with each other during the party, however, we can build a strategy by using the cupcake.
The strategy itself is pretty simple, we take one of the guests and make him a cupcake watcher... Essentially, whenever he is being chosen for the labyrinth exploration, he will look at the plate with the 
cupcake in the center of the labyrinth and if there are no cupcake, he will ask for one. He also will count each time he asks for a new cupcake and when that counter reaches the number of guests he will notify 
Minotaur that everyone has visited labyrinth. Now, the other part of the strategy is actually eating the cake. Basically, all guests, including cupcake watcher, must eat cupcake exactly once. So, when they visit
labyrinth and see cupcake, they eat it. If there is no cupcake when they visit, they leave everything as it is (unless it's a cupcake watcher). 

Runs:

| Guests (N)    | Runtime Best  | Runtime Worst |
| ------------- | ------------- | ------------- |
| 10            | 2.7317ms      | 6.9365ms      |
| 20            | 63.605ms      | 83.5886ms     |    
| 50            | 1.1211062s    | 4.32568s      |
| 100           | 10.4912194s   | 53.5142195s   |

Since there is a random choosing factor in our algorithm (Minotaur choses randomly who is going to explore the labyrinth), the time complexity could become quite nasty looking...

#2 Minotaur’s Crystal Vase

The Minotaur decided to show his favorite crystal vase to his guests in a dedicated showroom with a single door. He did not want many guests to gather around the vase 
and accidentally break it. For this reason, he would allow only one guest at a time into the showroom. He asked his guests to choose from one of three possible strategies for 
viewing the Minotaur’s favorite crystal vase: 

- 1 Any guest could stop by and check whether the showroom’s door is open at any time and try to enter the room. While this would allow the guests to roam around the castle 
and enjoy the party, this strategy may also cause large crowds of eager guests to gather around the door. A particular guest wanting to see the vase would also have no 
guarantee that she or he will be able to do so and when. 

- 2 The Minotaur’s second strategy allowed the guests to place a sign on the door indicating when the showroom is available. The sign would read “AVAILABLE” or 
“BUSY.” Every guest is responsible to set the sign to “BUSY” when entering the showroom and back to “AVAILABLE” upon exit. That way guests would not bother trying 
to go to the showroom if it is not available. 

- 3 The third strategy would allow the quests to line in a queue. Every guest exiting the room was responsible to notify the guest standing in front of the queue that the 
showroom is available. Guests were allowed to queue multiple times. Which of these three strategies should the guests choose? Please discuss the advantages 
and disadvantages. Implement the strategy/protocol of your choice where each guest is represented by 1 running thread. You can choose a concrete number for the number of guests or ask the 
user to specify it at the start. 

Solution:

In option one, we would need to deal with queues of guests who are trying to access room even if it's being currently occupied. In option three, after visiting a showroom, we would need to look for the person who is next in queue
around the whole castle in order to notify them about showroom availability. In option two, however, the approach to solving the problem is the simplest one. We can explore castle however we want without standing and waiting when the showroom become available and we also don't need to look for other people around castle to notify them about room availability. All we need to do is to create a attandance sheet in front of the showroom and ask everyone who enters the room to note their attendance on it. The sign on the room will tell other guests the room availability and the attendance sheet will help Minotaur to figure out when all guests visited the showroom.

| Guests (N)    | Runtime Best  | Runtime Worst |
| ------------- | ------------- | ------------- |
| 10            | 1.6681ms      | 2.259ms       |
| 20            | 2.5839ms      | 3.5777ms      |    
| 50            | 6.6367ms      | 8.7644ms      |
| 100           | 12.9362ms     | 15.031ms      |
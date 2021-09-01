Integer Sequences
-----------------
 - in some fractally sequences eg primes it appears as though the only reason it fucks up is from overflowing the rule string, so define sequences with a fn for all n where possible, or maybe just generate a really really big string? depends if we care about performance, probably not lol its pretty fast

 - dragon curve: always spaceship? also to generate iteratively, double it, R in middle, change last R to L or so, like L systems
    - look closer at spaceships hey. maybe it counts lol
 - counting integers rg RLRRLRRRLRRRRL...
 - powers of two
 - alternating fibonacci
   - could also try encoding it the other way a la counting
   - and counting alternating
 - pascals triangle sequences cool as well




Rule Construction
-----------------
 - conversely to above, there was one rule that was doing cool sierpinski triangle shit and if not for overflowing would seem to do it forever. SEQUENCE CONTINUATION. how to continue the sequence that i didnt know was a sequence? Well, you could just add Rs and Ls and see which one doesnt overflow
 - appending arbitrarily long sequence of Rs at the end...? wont matter if you can spaceship in under the period
   - similarly if you append RLRLRL...
   - maybe you dont get chaos if the higher orders things are some really regular cycle
 - repeating sequence is identical behaviour right
 - arbitrarily small change: sequence repition and then change on the end
 - ascending descending even odd etc. partial odd even. hilbert sequence or whatever fair turn in chess thing?
   - L systems to generate - i know im going to have to with dragon curve




Search Space Exploration
------------------------
 - classifying
   - growth rate
     - some super slow ones
     - some shit boring blob ones. chaos I guess I shouldn't really knock it too much lol
     - some blobs that want to spaceship but cant quite and you add one thing
   - spaceship period
     - criteria to space ship? eg balance of Rs and Ls
   - growing period: squares and triangles
   - symmetrical family
     - seems to do a circuit, would be cool to just trace it for a spaceship shape
     - always returns to the origin?
     - always RR / LL never R / L on their own
   - high number of Rs and Ls at start: gets really self contained / small growth rate I think
   - RL+ family usually interesting results
   - N/S is really bad for some reason, always makes chaotic behaviour
 
 - if you can come up with a metric that leads to interesting behaviour and then genetic algorithm it
    - minimizing time to overflow i think is a good one, definitely leads to spaceships though as well.
    - min overflow + max expansion area = square family or triangles etc. or maybe find interesting spaceships of high period.
    - actually maybe they are interesting if we get follow cam, maybe the long dragon curve one is cool. maybe you can get arbitrary periods
      by following dragon curve then. that would be an awesome result.
    
 - infintie growth with no repitition - that would be how you get the counting sequence and maybe fractals and shit


Usability
---------
 - interactive panning
 - follow ant eg for spaceships
 - r to reset
 - change rule at runtime
 - swap colour fn at runtime as well
   - do some full acid one that cycles through colours


Misc
----
 - Love how this relates to primes, fractals, L systems, etc
 - whats possible with No turns and U turns
   - the cool counter thing
 - related CAs: spawn reset
 - refactor: make a 'rule' which might just be a fn ptr or something
 - higher order rules like rule but if i > whatever RLRLRL... rule combinators
 - make a folder / file for storing rules, so its organized and then can target usability of swapping out during simulation
 - initial conditions: the sense in which the rule is set up to do something like some kind of spaceship, but it takes a lot of random churn before it will eventually arrive at the conditions to do so. I mean it happens on just RL.
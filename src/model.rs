// Exponential decay; Everytime a directory is visited, its score increases, but all other scores in the history are multiplied by a decay factor (e.g. 0.99)
// This ensures that a folder you used a 100 times last year doesn't outrank a folder you've just used 10 times this week. 
// Preventing a 'stale' feelink. Where old habits outweigh new ones.


// Markov chain.
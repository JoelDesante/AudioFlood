# Audio Flood
A procedural noise generator!

### Why Audio Flood
I created Audio Flood to solve a problem I experienced at night in my apartment. My upstairs neighbor has a very deep voice that carries through the walls of my 1960s era apartment building. I found my self having trouble falling asleep because the silence would always be broken by the faint indistiguishable mubling of my neighbor. 

My original soultion to this problem was to turn on the air conditioning unit which produced low rumbing noise that drowned out the sound of my neighbor. However, I quickly discovered that the cost of running my air condintioner all night was quite high.

Of course, the easy solution to this would be to record a sample of the air conditioner and then play the audio back on loop through a speaker with a high bass response. But I became intrested in procedural generation, so I decide I would build this application.

### What does Audio Flood do?
This application procedurally generates a customizable stream of audio which is meant to mimic the sounds produced by my air conditioner. My hope is that I can load this software on to a RaspberryPi which is connected to a bass speaker for a car. Then play the real time audio stream through the rig to cheaply emulate the noise dampening effects of my air conditioner at a reduced cost.

# Project status.
I have succesfully been able to generate a real time audio stream. However, the sound produced is not similar to the air conditioning unit in any way. I hope to learn more about fourier transforms so I can apply this knowledge to generate more complex frequencies.

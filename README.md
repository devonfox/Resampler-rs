## HW2: Resampling

CS410P - Music, Sound, and Computers

*Devon Fox 2022*
### Linux/Mac Build/Run Instructions

* This code was tested/run on Rust 1.57.0.  If using 1.59.0, it has not yet been tested.  You can check version by entering the following at the command-line:

        cargo version

* To run the program, there is a single required command-line argument, being the filename of the wav file to resample. Replace 'filename.wav' with the full path relative to the program folder for the wavfile you wish to use.

        cargo run 'filename.wav'

* This will create a new .wav file at half the sample rate, called 'rfilename.wav'.  

*This program will only accept mono (single channel) wav files.*

### What Went Down

* For this assignment, I used the 'hound' crate to facilitate the reading and writing of .wav files.  The program takes one command argument of the filename and then filters the samples with an FIR filter using the 91 filter coefficients provided by Bart.  It filters, then it writes every sample into a wav file, thus making the new wav file half the samplerate as the input wav file.  

###  How It Went

* It went well! I didn't implement many tests, but one thing I tested was to ensure the incoming wav input was mono ( single channel ) as the algorithm accounts for this only. I could potentially, make this work either way depending on the source wav file, however, perhaps I will upgrade this at another time. The filtering was easy enough using the provided filter coefficients, however, there was a small loss of the initial samples, about 'n' amount of filter coefficients to be exact.  In this manner, there is a tiny loss, but hey, how bad is 91 out of 48,000.  This was a fun exercise, and helped me understand a simple filter and the flow of samples.  I was trying to imagine how fast this processing the audio, and with 5 second runtime on my machine, if I had an audio buffer of 512 samples, maybe I'd be have around a 3ms latency. That math is really hit or miss, but you get the idea.  
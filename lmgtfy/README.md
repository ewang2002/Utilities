# lmgtfy
A small utility program that takes in a query and prints a **L**et **M**e **G**oogle **T**hat **F**or **Y**ou link *and* 
a Bit.ly link. Useful when your friends ask a question that can easily be asked on Google.

## Setup
This assumes a Windows OS. Steps may differ for Linux or Mac.

1. Make sure you get the release build of this executable. See the previous section for more information.
2. Get a Bit.ly access token [here](https://bitly.is/accesstoken). You will need a **free** account. 
3. In your user environmental variables, create the key `BITLY_API` with the value being your API key found in step 2.
4. Put the executable (from step 1) into a folder (preferably containing other executables/utilities). Then, in your 
user environmental variables, put the path to this folder under the variable `PATH`. 
5. You should be able to access `lmgtfy` from the CLI.

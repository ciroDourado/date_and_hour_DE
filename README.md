# date_and_hour_DE
A refactored version from my first attempt to make a "date and hour app". Check my other repository

This pretty little thing is a combination from two different rust packages I've developed in order to reorganize and clean some stuff about my other project with similar name.

Basically, to make this run as a clock with a calendar, we must declare and initialize a clock structure and a calendar too. Mount a loop to print them, iterate time (hours first, and verifying if the clock has completed to iterate the days), and freeze the thread. Nothing more. 

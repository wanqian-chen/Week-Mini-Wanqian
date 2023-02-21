# Week 5 Mini Project for IDS 721

## Introduction

This is a mini project which get data from `www.nba.com` and present it to users by connecting to `nba-api`.

## Usage

### Prerequisites

To run this application, you will need to install the required packages by running the following command:

```bash
pip install -r requirements.txt
```

Then you can run the application by running the following command:

```bash
python src/main.py
```

### Features

Main features of this application are:
1. Find Player by Name
2. Find Team
3. Quit

In the first feature, you can:
1. Find Player by Full Name
2. Find Player by First Name
3. Find Player by Last Name
4. Get All Players

In the second feature, you can:
1. Find Team by Full Name
2. Find Team by State
3. Find Team by City
4. Find Team by Nickname
5. Find Team by Foundation Year
6. Find Team by Abbreviation
7. Find Team by Id
8. Get All Teams

### Example

```bash
$ python src/main.py
1. Find Player by Name
2. Find Team
3. Quit
Enter your choice: 1
1. Find Player by Full Name
2. Find Player by First Name
3. Find Player by Last Name
4. Get All Players
Enter your choice: 3
Enter the last name of the player: curry
====================
Player Information
====================
Player Name:  Dell Curry
Player Id:  209
Player First Name:  Dell
Player Last Name:  Curry
Player Is Active:  False
--------------------
Player Name:  Eddy Curry
Player Id:  2201
Player First Name:  Eddy
Player Last Name:  Curry
Player Is Active:  False
--------------------
Player Name:  JamesOn Curry
Player Id:  201191
Player First Name:  JamesOn
Player Last Name:  Curry
Player Is Active:  False
--------------------
Player Name:  Michael Curry
Player Id:  688
Player First Name:  Michael
Player Last Name:  Curry
Player Is Active:  False
--------------------
Player Name:  Seth Curry
Player Id:  203552
Player First Name:  Seth
Player Last Name:  Curry
Player Is Active:  True
--------------------
Player Name:  Stephen Curry
Player Id:  201939
Player First Name:  Stephen
Player Last Name:  Curry
Player Is Active:  True
--------------------
Player Name:  Carey Scurry
Player Id:  78102
Player First Name:  Carey
Player Last Name:  Scurry
Player Is Active:  False
--------------------
```

## References

`nba-api`: [Github-nba-api](https://github.com/swar/nba_api)
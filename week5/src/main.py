from nba_api.stats.static import players
from nba_api.stats.static import teams

# custom_headers = {
#     'Host': 'stats.nba.com',
#     'Connection': 'keep-alive',
#     'Cache-Control': 'max-age=0',
#     'Upgrade-Insecure-Requests': '1',
#     'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.86 Safari/537.36',
#     'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3',
#     'Accept-Encoding': 'gzip, deflate, br',
#     'Accept-Language': 'en-US,en;q=0.9',
# }

# Find player by name
def find_player_by_name():
    # Print the menu
    print('''
    1. Find Player by Full Name
    2. Find Player by First Name
    3. Find Player by Last Name
    4. Get All Players
    ''')

    # Get the user's choice
    choice = input('Enter your choice: ')

    if (choice == '1'):  # Get the player by full name
        full_name = input('Enter the full name of the player: ')
        player = players.find_players_by_full_name(full_name)
    if (choice == '2'):  # Get the player by first name
        first_name = input('Enter the first name of the player: ')
        player = players.find_players_by_first_name(first_name)
    if (choice == '3'):  # Get the player by last name
        last_name = input('Enter the last name of the player: ')
        player = players.find_players_by_last_name(last_name)
    if (choice == '4'):  # Get all players
        player = players.get_players()
    
    print('====================')
    print('Player Information')
    print('====================')
    for p in player:
        print("Player Name: ", p['full_name'])
        print("Player Id: ", p['id'])
        print("Player First Name: ", p['first_name'])
        print("Player Last Name: ", p['last_name'])
        print("Player Is Active: ", p['is_active'])
        print("--------------------")
    # if no result
    if not player:
        print("No results found")

# Find team by name
def find_team_by_name():
    # Print the menu
    print('''
        1. Find Team by Full Name
        2. Find Team by State
        3. Find Team by City
        4. Find Team by Nickname
        5. Find Team by Foundation Year
        6. Find Team by Abbreviation
        7. Find Team by Id
        8. Get All Teams
    ''')

    # Get the user's choice
    choice = input('Enter your choice: ')

    if (choice == '1'):  # Get the team by full name
        full_name = input('Enter the full name of the team: ')
        team = teams.find_teams_by_full_name(full_name)
    if (choice == '2'):  # Get the team by state
        state = input('Enter the state of the team: ')
        team = teams.find_teams_by_state(state)
    if (choice == '3'):  # Get the team by city
        city = input('Enter the city of the team: ')
        team = teams.find_teams_by_city(city)
    if (choice == '4'):  # Get the team by nickname
        nickname = input('Enter the nickname of the team: ')
        team = teams.find_teams_by_nickname(nickname)
    if (choice == '5'):  # Get the team by foundation year
        year_founded = input('Enter the year the team was founded: ')
        team = teams.find_teams_by_year_founded(year_founded)
    if (choice == '6'):  # Get the team by abbreviation
        abbreviation = input('Enter the abbreviation of the team: ')
        team = teams.find_teams_by_abbreviation(abbreviation)
    if (choice == '7'):  # Get the team by id
        id = input('Enter the id of the team: ')
        team = teams.find_teams_by_id(id)
    if (choice == '8'):  # Get all teams
        team = teams.get_teams()
    
    print('====================')
    print('Team Information')
    print('====================')
    for t in team:
        print("Team Name: ", t['full_name'])
        print("Team Id: ", t['id'])
        print("Team Abbreviation: ", t['abbreviation'])
        print("Team Nickname: ", t['nickname'])
        print("Team City: ", t['city'])
        print("Team State: ", t['state'])
        print("Team Year Founded: ", t['year_founded'])
        print("--------------------")
    # if no result
    if not team:
        print("No results found")

def main():

    # Print the menu
    print('''
    1. Find Player by Name
    2. Find Team
    3. Quit
    ''')

    # Get the user's choice
    choice = input('Enter your choice: ')

    if (choice == '1'):  # Find the player by name
        find_player_by_name()
    elif (choice == '2'):  # Find the team by name
        find_team_by_name()
    elif (choice == '3'):  # Quit
        print('Goodbye!')
        exit()

# main
if __name__ == '__main__':
    main()
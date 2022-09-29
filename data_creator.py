number = 26
name = "bracket_in_comment"

open("src/test_data/" + str(number) + "_" + name + "_INPUT.qml", "w")
open("src/test_data/" + str(number) + "_" + name + "_OUTPUT.qml", "w")

print("\"" + str(number) + "_" + name + "\",")

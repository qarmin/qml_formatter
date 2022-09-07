number = 23
name = "connect_at_end_line"


open("src/test_data/" + str(number) + "_" + name + "_INPUT.qml", "w")
open("src/test_data/" + str(number) + "_" + name + "_OUTPUT.qml", "w")

print("\"" + str(number) + "_" + name + "\",")

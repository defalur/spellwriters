extends Node

class_name TurnQueue

var active_character = 0
var characters = []

func add_character(id):
	characters.append(id)

func end_turn():
	print("End turn")
	active_character = (active_character + 1) % len(characters)

func get_active_player():
	return active_character

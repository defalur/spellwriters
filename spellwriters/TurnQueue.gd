extends Node

class_name TurnQueue

var active_character

func initialize():
	active_character = get_child(0)

func end_turn():
	print("End turn")
	var new_index : int = (active_character.get_index() + 1) % get_child_count()
	active_character = get_child(new_index)

func get_active_player():
	return active_character

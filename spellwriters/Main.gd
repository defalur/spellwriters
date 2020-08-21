extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var turn_queue
var world
var moving: bool = true


# Called when the node enters the scene tree for the first time.
func _ready():
	turn_queue = get_node("TurnQueue")
	turn_queue.initialize()
	world = get_node("World")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.pressed:
			if moving:
				var player = turn_queue.get_active_player()
				player.move_to(world.snap_to_map(event.position))

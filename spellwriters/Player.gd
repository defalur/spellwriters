extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var max_health = 10
onready var health = 10
var entity_id


# Called when the node enters the scene tree for the first time.
func _ready():
	get_node("HealthBar").value = health
	entity_id = get_tree().current_scene.request_id(position)
	print("Hello!")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func move_to(grid_pos, world):
	print("Moving to: " + str(grid_pos))
	if world.check_move(grid_pos):
		position = world.move(entity_id, grid_pos)


func _on_Player_area_entered(area):
	var spell = area.get_parent()
	health -= spell.get_damage()
	get_node("HealthBar").value = health

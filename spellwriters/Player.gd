extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var max_health = 10
onready var health = 10


# Called when the node enters the scene tree for the first time.
func _ready():
	get_node("HealthBar").value = health
	print("Hello!")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func move_to(pos):
	print("Moving to: " + str(pos))
	position = pos


func _on_Player_area_entered(area):
	var spell = area.get_parent()
	health -= spell.get_damage()
	get_node("HealthBar").value = health

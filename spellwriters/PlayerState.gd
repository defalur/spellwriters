extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

enum CELL_TYPES { ACTOR, OBJECT, OBSTACLE }
export(CELL_TYPES) var type = CELL_TYPES.ACTOR

export(int) var max_health = 10
onready var health = 10
var id

var moving = false
var target_pos

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Hi!")
	get_node("HealthBar").value = health
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if moving:
		var map = get_parent()
		position = map.request_move(self, target_pos)
		print("Move to ", position)
		moving = false
	pass

func init_move(target):
	moving = true
	target_pos = target

func take_damage(dmg):
	health -= dmg

func update_display():
	get_node("HealthBar").value = health

extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var max_time = 3
var cur_time = 0

var explosiveness = {
	"fire": 0,
	"stone": 1
}

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Area Spell ready!")
	get_node("Particles2D").emitting = true
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if cur_time >= max_time:
		queue_free()
	cur_time += delta


func change_mat(material):
	print(material)
	get_node("Particles2D").texture = load(
		"res://sprites/particles/" + material + ".png"
	)
	get_node("Particles2D").explosiveness = explosiveness[material]

func target(origin: Vector2, target: Vector2):
	position = target

func get_damage():
	return 1
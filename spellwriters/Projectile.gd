extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var speed = 128
var dir
var max_dist
onready var dist = 0
var active = false

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Projectile ready!")
	#get_node("Area2D/CollisionShape2D").disabled = true
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if dist >= max_dist:
		queue_free()
	var d_position = dir * delta * speed
	dist += d_position.length()
	position += d_position

func change_mat(material):
	print(material)
	get_node("Sprite").texture = load(
		"res://sprites/materials/" + material + ".png"
	)

func target(origin: Vector2, target: Vector2):
	position = origin
	max_dist = origin.distance_to(target)
	dir = (target - origin).normalized()

func get_damage():
	if active:
		return 2
	else:
		return 0

func _on_Area2D_area_exited(area):
	#print("Enabling damage")
	active = true
	#get_node("Area2D/CollisionShape2D").disabled = false

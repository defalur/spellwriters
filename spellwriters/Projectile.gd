extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var speed = 128
var dir
var max_dist
onready var dist = 0
var active = false
var entity_map

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Projectile ready!")
	#get_node("Area2D/CollisionShape2D").disabled = true
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if dist >= max_dist:
		entity_map.update_display()
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
	return 2

func get_hits(entities_map : TileMap):
	entity_map = entities_map

	var grid_pos = entities_map.world_to_map(position)
	var grid_max_dist = max_dist / entities_map.cell_size.x
	var hits = entities_map.raymarch_grid(grid_pos, dir, grid_max_dist)
	if len(hits) == 0:
		return []
	var final_hit = hits[0]
	print(hits)
	for h in hits:
		if h.distance_to(grid_pos) < final_hit.distance_to(grid_pos): #not optimal in the slightest
			final_hit = h
	max_dist = (entities_map.map_to_world(final_hit)
		+ entities_map.cell_size / 2).distance_to(position)
	return [final_hit]

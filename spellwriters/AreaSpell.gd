extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var max_time = 3
var cur_time = 0
var entity_map

const neighbours = [
	Vector2(-1, 0),
	Vector2(0, -1),
	Vector2(1, 0),
	Vector2(0, 1)
]

var explosiveness = {
	"fire": 0,
	"stone": 1
}

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Area Spell ready!")
	entity_map.update_display()
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if cur_time >= max_time:
		queue_free()
	cur_time += delta


func change_mat(material):
	print(material)
	for c in get_children():
		print(c.position)
		c.emitting = true
		c.texture = load(
			"res://sprites/particles/" + material + ".png"
		)
		c.explosiveness = explosiveness[material]

func target(origin: Vector2, target: Vector2):
	position = target

func get_damage():
	return 1

func get_hits(entities_map: TileMap):
	entity_map = entities_map

	var grid_pos = entities_map.world_to_map(position)

	var result = []
	if entities_map.get_cellv(grid_pos) != -1:
		result.append(grid_pos)
	for n in neighbours:
		if entities_map.get_cellv(grid_pos + n) != -1:
			result.append(grid_pos + n)
		var n_pos = entities_map.map_to_world(grid_pos + n) \
			+ entities_map.cell_size / 2

	return result

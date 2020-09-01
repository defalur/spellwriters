extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var tilemap: TileMap
var entity_id = 0
var entities : Dictionary

# Called when the node enters the scene tree for the first time.
func _ready():
	tilemap = get_node("TileMap")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func snap_to_map(position):
	var pos = tilemap.world_to_map(position)
	return tilemap.map_to_world(pos) + tilemap.cell_size / 2

func world_to_map(position):
	return tilemap.world_to_map(position)

func map_to_world(position):
	return tilemap.map_to_world(position) + tilemap.cell_size / 2

func register_entity(position):
	var id = entity_id
	entity_id += 1
	entities[id] = position
	return id

func check_move(grid_pos):
	for p in entities.values():
		if p == grid_pos:
			return false
	return true

func move(id, grid_pos):
	entities[id] = grid_pos
	return map_to_world(grid_pos)

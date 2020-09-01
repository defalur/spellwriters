extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

enum CELL_TYPES { ACTOR, OBJECT, OBSTACLE }

var tilemap: TileMap
var entities: TileMap
var entity_id = 0

# Called when the node enters the scene tree for the first time.
func _ready():
	tilemap = get_node("TileMap")
	entities = get_node("EntitiesMap")


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

func register_player(position):
	return register_entity(position, "player")

func register_entity(position, type):
	var id = entity_id
	entity_id += 1
	entities.add_entity(position, id, type)
	return id

func move(id, grid_pos):
	get_node("EntitiesMap").move_entity(id, grid_pos)

func get_entity_pos(id):
	return get_node("EntitiesMap").get_entity_pos(id)

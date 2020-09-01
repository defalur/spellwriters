extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var tilemap: TileMap


# Called when the node enters the scene tree for the first time.
func _ready():
	tilemap = get_node("TileMap")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func snap_to_map(position):
	var pos = tilemap.world_to_map(position)
	return tilemap.map_to_world(pos) + tilemap.cell_size / 2

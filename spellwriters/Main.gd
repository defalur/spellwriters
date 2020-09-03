extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var turn_queue
var world
var moving: bool = true
var cur_spell


# Called when the node enters the scene tree for the first time.
func _ready():
	#print(self)
	turn_queue = get_node("TurnQueue")
	world = get_node("World")
	turn_queue.add_character(world.register_player(Vector2(0,0)))
	turn_queue.add_character(world.register_player(Vector2(0,1)))


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func validate_spell():
	var spell_str = get_node("MainUi/SpellBox").text
	var spell = get_node("Spellparser").compile_spell(spell_str)
	cur_spell = load("res://Spell.tscn").instance()
	if not cur_spell.set_data(spell, 0):
		return
	add_child(cur_spell)
	moving = false
	print(spell)

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.pressed:
			if moving:
				var player = turn_queue.get_active_player()
				var target_pos = world.world_to_map(event.position)
				
				world.move(player, target_pos)
			else:
				var player = turn_queue.get_active_player()
				var player_pos = world.get_entity_pos(player)
				cur_spell.set_cast_positions(
					world.world_to_map(player_pos),
					[world.world_to_map(event.position)]
				)
				cur_spell.cast(world)
				moving = true
				cur_spell = null

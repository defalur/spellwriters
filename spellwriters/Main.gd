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
	turn_queue.initialize()
	world = get_node("World")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func validate_spell():
	var spell_str = get_node("MainUi/SpellBox").text
	var spell = get_node("Spellparser").compile_spell(spell_str)
	cur_spell = load("res://Spell.tscn").instance()
	cur_spell.set_data(spell, 0)
	add_child(cur_spell)
	moving = false
	print(spell)

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.pressed:
			if moving:
				var player = turn_queue.get_active_player()
				player.move_to(world.snap_to_map(event.position))
			else:
				var player = turn_queue.get_active_player()
				cur_spell.set_cast_positions(
					player.position,
					[world.snap_to_map(event.position)]
				)
				cur_spell.cast()
				moving = true
				cur_spell = null

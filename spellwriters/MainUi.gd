extends Control


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

signal end_turn
signal validate_spell

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_end_turn():
	emit_signal("end_turn")
	reset_spell()

func reset_spell():
	var spell_box = get_node("SpellBox")
	spell_box.text = ""


func _on_validate_spell():
	emit_signal("validate_spell")

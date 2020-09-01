extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

class SpellElement:
	var shape #name of shape
	var trigger
	var targets
	var effects: Array #vector of effect
	var materials: Array #vector of materials

var cur_element
var last_i
var cast_origin
var cast_targets

var spells = {
	"projectile": preload("res://Projectile.tscn"),
	"area": preload("res://AreaSpell.tscn")
}

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func parse_effect(data, i):
	if data[i] != "spelleffect":
		return null
	i += 1
	var effect = data[i]
	i += 1
	var mat = data[i]
	i += 1
	
	return [[effect, mat], i]

func parse_element(data, i):
	if data[i] != "spellelement":
		return null
	i += 1
	if data[i] != "shaperune":
		return null
	i += 1
	var result = SpellElement.new()
	result.shape = data[i]
	i += 1
	result.trigger = data[i]
	i += 1
	result.targets = data[i]
	i += 2#jump over the end token
	print(data[i])
	while data[i] == "spelleffect":
		var effect_ret = parse_effect(data, i)
		var effect = effect_ret[0]
		i = effect_ret[1]
		result.effects.append(effect[0])
		result.materials.append(effect[1])
		i += 1
	return [result, i]

func set_data(data, i):
	if data[i] == "spell":
		i += 1
	elif data[i] != "spellelement":
		return

	var element_i = parse_element(data, i)
	cur_element = element_i[0]
	last_i = element_i[1]

func set_cast_positions(origin, targets):
	cast_origin = origin
	cast_targets = targets

func cast(world):
	print("Casting spell!")
	var main_node = get_parent()
	for p in cast_targets:
		p = world.map_to_world(p)
		var cur_spell = spells[cur_element.shape].instance()
		cur_spell.target(world.map_to_world(cast_origin), p)
		#print(len(cur_element.materials))
		if len(cur_element.materials) > 0:
			cur_spell.change_mat(cur_element.materials[0])
		#print(main_node)
		main_node.add_child(cur_spell)
	queue_free()

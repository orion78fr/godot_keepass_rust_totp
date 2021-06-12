extends VBoxContainer

export var timer: float setget set_timer

func set_timer(p_timer: float):
	timer = p_timer

	for n in get_children():
		n.progress(timer)

func update_otp():
	for n in get_children():
		n.generate()

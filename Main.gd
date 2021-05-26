extends Control

func _ready():
	$ScrollContainer.rect_size = OS.window_size
	$"ScrollContainer/VBoxContainer/Button".text = KeepassTotp.test()

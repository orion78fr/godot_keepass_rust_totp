extends Control

func _ready():
	var safe_area = OS.get_window_safe_area()

	$ScrollContainer.rect_position = safe_area.position
	$ScrollContainer.rect_size = safe_area.size

	$FileDialog.popup()


func _on_FileDialog_file_selected(path):
	$"Debug Label".text = KeepassTotp.new().open_keepass_db(path, "azerty")

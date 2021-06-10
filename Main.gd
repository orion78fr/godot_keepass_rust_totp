extends Control

var totp;

func _ready():
	var safe_area = OS.get_window_safe_area()

	$ScrollContainer.rect_position = safe_area.position
	$ScrollContainer.rect_size = safe_area.size

	$FileDialog.popup()


func _on_FileDialog_file_selected(path):
	totp = KeepassTotp.new()

	var error = totp.open_keepass_db(path, "azerty")
	if error != null :
		$DebugLabel.text = error
		return

	var generated = totp.generate_otps()
	if generated == null || generated.size() == 0:
		$DebugLabel.text = "No OTPs generated"
		return

	var s = ""
	for g in generated :
		s += g + "\n"

	$DebugLabel.text = s

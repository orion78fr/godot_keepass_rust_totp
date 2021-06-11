extends Control

var totps

onready var OtpButton = preload("res://OtpButton.tscn")

func _ready():
	var safe_area = OS.get_window_safe_area()

	$ScrollContainer.rect_position = safe_area.position
	$ScrollContainer.rect_size = safe_area.size

	var os_name = OS.get_name()

	if os_name == "Android" || os_name == "HTML5":
		_on_FileDialog_file_selected("res://test/totp_test.kdbx")
	else:
		$FileDialog.popup()


func _on_FileDialog_file_selected(path):
	var native = KeepassTotp.new()

	var res = native.open_keepass_db(path, "azerty");

	if res.has("Err"):
		$DebugLabel.text = res.get("Err")
	else:
		totps = res.get("Ok")

		for totp in totps:
			var button = OtpButton.instance()
			button.set_otp(totp)
			$"ScrollContainer/VBoxContainer".add_child(button)



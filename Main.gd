extends Control

var totps

onready var OtpButton = preload("res://OtpButton.tscn")

var file

func _ready():
	var safe_area = OS.get_window_safe_area()

	$ScrollContainer.rect_position = safe_area.position
	$ScrollContainer.rect_size = safe_area.size

	var os_name = OS.get_name()

	if os_name == "Android":
		var singleton = Engine.get_singleton("Android File Opener Plugin")
		$DebugLabel.text = singleton.getTheHelloWorld()

		var readFile = singleton.getKeepassFile()

		# Copy to a temp location
		file = "user://tempFile"
		var f = File.new()
		f.open(file, File.WRITE)
		f.store_buffer(readFile)
		f.close()

		$PasswordDialog.popup()
	elif os_name == "HTML5":
		file = "res://test/totp_test.kdbx"
		open_database("azerty")
	else:
		$FileDialog.popup()

func _on_ErrorDialog_popup_hide():
	$FileDialog.popup()

func _on_FileDialog_file_selected(path):
	file = path
	$PasswordDialog.popup()

func _on_LineEdit_text_entered(pwd):
	$PasswordDialog.hide()
	open_database(pwd)

func _on_Password_Button_pressed():
	$PasswordDialog.hide()
	var pwd = $"PasswordDialog/CenterContainer/HBoxContainer/LineEdit".text
	open_database(pwd)

func open_database(pwd):
	var native = KeepassTotp.new()
	var res = native.open_keepass_db(file, pwd);

	if res.has("Err"):
		$"ErrorDialog/CenterContainer/Label".text = res.get("Err")
		$ErrorDialog.popup()
	else:
		totps = res.get("Ok")

		$AnimationPlayer.play("Timer")

		for totp in totps:
			var button = OtpButton.instance()
			button.set_otp(totp)
			$"ScrollContainer/VBoxContainer".add_child(button)

extends Control

onready var native = KeepassTotp.new()
onready var OtpButton = preload("res://OtpButton.tscn")

var totps
var file
var bytes

func _ready():
	var safe_area = OS.get_window_safe_area()

	$ScrollContainer.rect_position = safe_area.position
	$ScrollContainer.rect_size = safe_area.size

	var os_name = OS.get_name()

	if os_name == "Android":
		$AdParent.position.y = safe_area.position.y
		
		if !Engine.has_singleton("Android Helper Plugin"):
			$DebugLabel.text = "Cannot find the android plugin"
			return
		
		var singleton = Engine.get_singleton("Android Helper Plugin")
		
		singleton.connect("file_found", self, "android_file_open")
		
		singleton.openKeepassFile()
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
	var res;
	
	if bytes != null:
		res = native.read_keepass_db(bytes, pwd)
	else:
		res = native.open_keepass_db(file, pwd)

	if res.has("Err"):
		$"ErrorDialog/CenterContainer/Label".text = res.get("Err")
		$ErrorDialog.popup()
	else:
		totps = res.get("Ok")
		
		display_totps()

func android_file_open(res):
	bytes = res
	$PasswordDialog.popup()

func display_totps():
	$AnimationPlayer.play("Timer")

	for totp in totps:
		var button = OtpButton.instance()
		button.set_otp(totp)
		$"ScrollContainer/VBoxContainer".add_child(button)
	
	
	$AdParent/AdMob.load_banner()

func _on_AdMob_banner_loaded():
	$AdParent/AdMob.show_banner()
	$ScrollContainer.rect_position.y = OS.get_window_safe_area().position.y + $AdParent/AdMob.get_banner_dimension().y
	$ScrollContainer.rect_size.y = OS.get_window_safe_area().size.y - $AdParent/AdMob.get_banner_dimension().y

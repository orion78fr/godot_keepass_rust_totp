extends Control

func _ready():
	$ScrollContainer.rect_size = OS.window_size
	$"ScrollContainer/VBoxContainer/Button".text = KeepassTotp.new().test()
	
	KeepassTotp.new().open_keepass_db("test/totp_test.kdbx", "azerty")

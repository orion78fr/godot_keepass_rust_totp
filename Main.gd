extends Control

func _ready():
	$ScrollContainer.rect_size = OS.window_size
	$"Debug Label".text = KeepassTotp.new().open_keepass_db("res://test/totp_test.kdbx", "azerty")
	

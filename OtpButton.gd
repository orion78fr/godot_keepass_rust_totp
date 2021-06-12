extends Control

var otp

func set_otp(p_otp):
	otp = p_otp

	var custom_icon = otp.get_custom_icon();
	if custom_icon != null :
		var image = Image.new()
		image.load_png_from_buffer(custom_icon)

		var texture = ImageTexture.new()
		texture.create_from_image(image)

		$"HBoxContainer/CenterContainer/TextureProgress".texture_under = texture
		$"HBoxContainer/CenterContainer/TextureProgress".texture_progress = texture

	$"HBoxContainer/Data/Name".text = otp.name
	$"HBoxContainer/Data/User".text = otp.user
	$"HBoxContainer/Data/URL".text = otp.url

	generate()

func generate():
	var generated: String = otp.generate()
	$"HBoxContainer/OTP".text = generated.insert(3, " ")

func progress(value):
	$"HBoxContainer/CenterContainer/TextureProgress".value = value

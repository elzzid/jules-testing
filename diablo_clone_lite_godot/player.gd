extends CharacterBody2D

@export var speed: float = 300.0

func _physics_process(_delta: float) -> void:
    var direction := Vector2.ZERO

    # Keyboard input (WASD)
    if Input.is_physical_key_pressed(KEY_W):
        direction.y -= 1.0
    if Input.is_physical_key_pressed(KEY_S):
        direction.y += 1.0
    if Input.is_physical_key_pressed(KEY_A):
        direction.x -= 1.0
    if Input.is_physical_key_pressed(KEY_D):
        direction.x += 1.0

    # Gamepad input
    # In Godot, joy_id 0 is typically the first connected gamepad.
    var joy_id := 0
    if Input.get_connected_joypads().size() > 0:
        joy_id = Input.get_connected_joypads()[0]

        # Left Stick
        var left_stick_x := Input.get_joy_axis(joy_id, JOY_AXIS_LEFT_X)
        var left_stick_y := Input.get_joy_axis(joy_id, JOY_AXIS_LEFT_Y)
        var left_stick := Vector2(left_stick_x, left_stick_y)

        var deadzone := 0.1
        if left_stick.length() > deadzone:
            # Remove deadzone from length and normalize
            var length := (left_stick.length() - deadzone) / (1.0 - deadzone)
            direction += left_stick.normalized() * length

        # D-pad
        if Input.is_joy_button_pressed(joy_id, JOY_BUTTON_DPAD_UP):
            direction.y -= 1.0
        if Input.is_joy_button_pressed(joy_id, JOY_BUTTON_DPAD_DOWN):
            direction.y += 1.0
        if Input.is_joy_button_pressed(joy_id, JOY_BUTTON_DPAD_LEFT):
            direction.x -= 1.0
        if Input.is_joy_button_pressed(joy_id, JOY_BUTTON_DPAD_RIGHT):
            direction.x += 1.0

    if direction.length_squared() > 0.0:
        # Normalize to prevent faster diagonal movement when using keyboard
        direction = direction.limit_length(1.0)

    velocity = direction * speed
    move_and_slide()

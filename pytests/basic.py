"""
Two QEMU devices linked to each other
"""

from simulator import *

LINK_IN = 1
LINK_OUT = 2

simulator_load_flash_image("../../tpp-internet4all/build/qemu_flash_image.bin")

link_a = link_create()
link_b = link_create()


device_a = qemu_create_instance()

qemu_connect_link_output(device_a, link_a)

# Add some custom config variables to tell the device which link is which
# This is completely arbitrary and they do not affect the simulator in any
# way.
qemu_set_config_word(device_a, LINK_IN, link_a)  # Read from link a
qemu_set_config_word(device_a, LINK_OUT, link_b)  # write to link b


device_b = qemu_create_instance()

qemu_connect_link_output(device_b, link_b)

qemu_set_config_word(device_b, LINK_IN, link_b)  # Read from link b
qemu_set_config_word(device_b, LINK_OUT, link_a)  # write to link a


simulator_launch()

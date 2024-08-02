"""
A single QEMU device linked to itself (echo).
"""

from simulator import *

LINK_IN = 1
LINK_OUT = 2

simulator_load_flash_image("./single_device.bin")

link_a = link_create()


device_a = qemu_create_instance()

# Whenever a device sends something to link A it will reach device A
# Any device can send data to any link but only a single device can
# receive data from a given link.
# Links work in a 'multiple-producer, single-consumer' fashion.
qemu_connect_link_output(device_a, link_a)

# Add some custom config variables to tell the device which link is which
# This is completely arbitrary and they do not affect the simulator in any
# way.
qemu_set_config_word(device_a, LINK_IN, link_a)  # Read from link a
qemu_set_config_word(device_a, LINK_OUT, link_a)  # write to link a


simulator_launch()

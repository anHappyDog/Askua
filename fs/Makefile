include ../include.mk


TMP_DIR := tmp


.PHONY:ext4 fat32
ext4:
	mkdir -p $(TMP_DIR)
	dd if=/dev/zero of=$(ROOT_FSIMG) bs=1M count=256
	mkfs.ext4 $(ROOT_FSIMG)
	mount  $(ROOT_FSIMG) $(TMP_DIR)
	cp ext4/* -r $(TMP_DIR)
	umount $(TMP_DIR)

fat32:
	mkdir -p $(TMP_DIR)	
	dd if=/dev/zero of=$(ROOT_FSIMG) bs=1M count=256
	mkfs.vfat -F 32 $(ROOT_FSIMG)
	mount  $(ROOT_FSIMG) $(TMP_DIR)
	cp fat32/* -r $(TMP_DIR)
	umount $(TMP_DIR)
clean:
	-rm *.img $(TMP_DIR) -rf

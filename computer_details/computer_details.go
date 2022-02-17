package main

import (
	"fmt"
	"io/ioutil"
	"net"
	"os"
	"runtime"
	"syscall"
	"time"
	"unsafe"
)

type (
	computer_details struct { // use method when calling functions
		hostname string; // done
		disks []disk_info; // done
		cpu_count int; // done
		mac_address string; // done
		os_version string;
		drives []string; // done
		utc string;
	}
	disk_info struct {
		disk_name string;
		disk_size uint64;
	}
	interface_info struct{
		name string;
		mac_address string;
	}
)

var (
	disk_array []disk_info
	interface_array []interface_info
	drive_array []string
	time_now = time.Now()
	//computer_details_struct computer_details
)

const (
	BLKGETSIZE64 = 0x80081272 // edode : blockdev --getsize64 /dev/sdb
)

func return_disk_info(fd, request, argp uintptr) (err error) {
	_, _, errno := syscall.Syscall(syscall.SYS_IOCTL, fd, request, argp)
	if errno != 0 {
		err = errno
	}
	return os.NewSyscallError("ioctl", err)
}

func (dl disk_info) get_disk_size_linux()(bool) { // lisandro : maybe call computer_details instead of disk_info
	files, err := ioutil.ReadDir("/sys/block")
	if err != nil {
		return false;
	}

	for _, f := range files {
		disk, err := os.Open("/dev/" + f.Name())
		if err != nil {
			continue;
		}

		var size uint64
		if err := return_disk_info(disk.Fd(), BLKGETSIZE64, uintptr(unsafe.Pointer(&size))); err != nil {
			continue;
		}
		dl.disk_size = size / 1024 / 1024 / 1024
		dl.disk_name = f.Name()

		disk_array = append(disk_array, dl)
		err = disk.Close()
		if err != nil {
			return false;
		}
	}
	return true;
}

func get_cpu_count()(int) {
	return runtime.NumCPU();
}

func get_hostname()(string) {
	hostname, err := os.Hostname()
	if err != nil {
		return "No hostname found";
	}
	return hostname;
}

func get_mac_address() (bool) {
	iface, err := net.Interfaces()
	if err != nil {
		return false;
	}
	var ii interface_info
	for _, ifa := range iface {
		a := ifa.HardwareAddr.String()
		if a != "" {
			ii.name = ifa.Name
			ii.mac_address = a
			interface_array = append(interface_array, ii)
		}
	}
	return false;
}

func get_drives() () {
	for _, drive := range "ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
		f, err := os.Open(string(drive) + ":\\")
		if err == nil {
			drive_array = append(drive_array, string(drive))
			f.Close()
		}
	}
	return;
}

func main()(){
	// add os version : https://stackoverflow.com/questions/44363911/detect-windows-version-in-go
	// look at : https://github.com/hdm/nextnet
	disk_info{}.get_disk_size_linux()
	fmt.Println("hello")
}


































#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <errno.h>
#include <string.h>

#define DEVICE_PATH "/dev/xiaomi-touch"

#define Touch_Game_Mode 0
#define Touch_Active_MODE 1
#define Touch_UP_THRESHOLD 2
#define Touch_Tolerance 3
#define Touch_Aim_Sensitivity 4
#define Touch_Tap_Stability 5
#define Touch_Expert_Mode 6
#define Touch_Edge_Filter 7
#define Touch_Panel_Orientation 8
#define Touch_Report_Rate 9
#define Touch_Fod_Enable 10
#define Touch_Aod_Enable 11
#define Touch_Resist_RF 12
#define Touch_Idle_Time 13
#define Touch_Doubletap_Mode 14
#define Touch_Grip_Mode 15
#define Touch_FodIcon_Enable 16
#define Touch_Nonui_Mode 17
#define Touch_Debug_Level 18
#define Touch_Power_Status 19
#define Touch_Pen_ENABLE 20
#define Touch_Mode_NUM 21

void print_usage(const char *prog_name);

int main(int argc, char *argv[]) {
    if (argc < 3) {
        print_usage(argv[0]);
        return EXIT_FAILURE;
    }

	int TOUCH_MAGIC = 21504;
    int mode = atoi(argv[2]);
    int value, mode_cmd;

    if (strcmp(argv[1], "--enable") == 0) {
		if (mode == 2)
			value = 4;
		else if (mode == 3)
			value = 4;
		else if (mode == 4)
			value = 4;
		else if (mode == 5)
			value = 4;
		else
			value = 1;		

    } else if (strcmp(argv[1], "--disable") == 0) {
        value = 0;
    } else {
        print_usage(argv[0]);
        return EXIT_FAILURE;
    }

    if (mode == 0 && value == 0)
        mode_cmd = 6;
	else
		mode_cmd = 0;

	int TOUCH_IOC_SETMODE = TOUCH_MAGIC + mode_cmd;
    int fd = open(DEVICE_PATH, O_RDWR);
	int arg[2] = { mode , value };
    if (fd < 0) {
        perror("Failed to open device file");
        return EXIT_FAILURE;
    }

    int ret;

    switch (mode) {
        case Touch_Game_Mode:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set game mode");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Game mode set to %d\n", value);
            break;

        case Touch_Active_MODE:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set active mode");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Active mode set to %d\n", value);
            break;

        case Touch_UP_THRESHOLD:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set UP threshold");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("UP threshold set to %d\n", value);
            break;

        case Touch_Tolerance:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set tolerance");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Tolerance set to %d\n", value);
            break;

        case Touch_Aim_Sensitivity:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set tolerance");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Aim_Sensitivity set to %d\n", value);
            break;

        case Touch_Tap_Stability:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set tolerance");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Tap_Stability set to %d\n", value);
            break;

        case Touch_Expert_Mode:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set tolerance");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Expert_Mode set to %d\n", value);
            break;

        case Touch_Edge_Filter:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set edge filter");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Edge filter set to %d\n", value);
            break;

        case Touch_Panel_Orientation:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set panel orientation");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Panel orientation set to %d\n", value);
            break;

        case Touch_Report_Rate:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set report rate");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Report rate set to %d\n", value);
            break;

        case Touch_Fod_Enable:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set FOD enable");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("FOD enable set to %d\n", value);
            break;

        case Touch_Aod_Enable:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set AOD enable");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("AOD enable set to %d\n", value);
            break;

        case Touch_Resist_RF:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set resist RF");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Resist RF set to %d\n", value);
            break;

        case Touch_Idle_Time:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set idle time");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Idle time set to %d\n", value);
            break;

        case Touch_Doubletap_Mode:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set double tap mode");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Double tap mode set to %d\n", value);
            break;

        case Touch_Grip_Mode:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set grip mode");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Grip mode set to %d\n", value);
            break;

        case Touch_FodIcon_Enable:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set FOD icon enable");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("FOD icon enable set to %d\n", value);
            break;

        case Touch_Nonui_Mode:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set non-UI mode");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Non-UI mode set to %d\n", value);
            break;

        case Touch_Debug_Level:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set debug level");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Debug level set to %d\n", value);
            break;

        case Touch_Power_Status:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set power status");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Power status set to %d\n", value);
            break;

        case Touch_Pen_ENABLE:
            ret = ioctl(fd, TOUCH_IOC_SETMODE, &arg);
            if (ret < 0) {
                perror("Failed to set pen enable");
                close(fd);
                return EXIT_FAILURE;
            }
            printf("Pen enable set to %d\n", value);
            break;

        default:
            fprintf(stderr, "Unknown mode: %d\n", mode);
            print_usage(argv[0]);
            close(fd);
            return EXIT_FAILURE;
    }

    close(fd);
    return EXIT_SUCCESS;
}

void print_usage(const char *prog_name) {
    fprintf(stderr, "Usage: %s <--enable|--disable> <mode>\n", prog_name);
    fprintf(stderr, "Modes:\n");
    fprintf(stderr, "  0  - Touch_Game_Mode\n");
    fprintf(stderr, "  1  - Touch_Active_MODE\n");
    fprintf(stderr, "  2  - Touch_UP_THRESHOLD\n");
    fprintf(stderr, "  3  - Touch_Tolerance\n");
    fprintf(stderr, "  4  - Touch_Aim_Sensitivity\n");
    fprintf(stderr, "  5  - Touch_Tap_Stability\n");
    fprintf(stderr, "  6  - Touch_Expert_Mode\n");
    fprintf(stderr, "  7  - Touch_Edge_Filter\n");
    fprintf(stderr, "  8  - Touch_Panel_Orientation\n");
    fprintf(stderr, "  9  - Touch_Report_Rate\n");
    fprintf(stderr, "  10 - Touch_Fod_Enable\n");
    fprintf(stderr, "  11 - Touch_Aod_Enable\n");
    fprintf(stderr, "  12 - Touch_Resist_RF\n");
    fprintf(stderr, "  13 - Touch_Idle_Time\n");
    fprintf(stderr, "  14 - Touch_Doubletap_Mode\n");
    fprintf(stderr, "  15 - Touch_Grip_Mode\n");
    fprintf(stderr, "  16 - Touch_FodIcon_Enable\n");
    fprintf(stderr, "  17 - Touch_Nonui_Mode\n");
    fprintf(stderr, "  18 - Touch_Debug_Level\n");
    fprintf(stderr, "  19 - Touch_Power_Status\n");
    fprintf(stderr, "  20 - Touch_Pen_ENABLE\n");
}


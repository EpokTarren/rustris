char get_key_c(void);

#if defined(WIN32) || defined(_WIN32) || defined(__WIN32__) || defined(__NT__)

#include <stdio.h>
#include <conio.h>

char get_key_c(void)
{
	if(kbhit()) return getch();

	return '\0';
}

#else

#include <stdio.h>
#include <termios.h>
#include <unistd.h>
#include <fcntl.h>

char get_key_c(void)
{
	struct termios oldt;

	tcgetattr(STDIN_FILENO, &oldt);

	struct termios newt = oldt;
	newt.c_lflag &= ~(ICANON | ECHO);
	
	tcsetattr(STDIN_FILENO, TCSANOW, &newt);
	
	int oldf = fcntl(STDIN_FILENO, F_GETFL, 0);
	fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK);

	char ch = getchar();

	tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
	fcntl(STDIN_FILENO, F_SETFL, oldf);

	if(ch != EOF) return ch;

	return '\0';
}

#endif
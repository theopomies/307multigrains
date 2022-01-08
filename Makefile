##
## EPITECH PROJECT, 2021
## makefile
## File description:
## 307multigrains
##

MAKEFLAGS	+=	--no-print-directory -j
BINARY_PATH	=	target/release/multigrains
BINARY_NAME =	307multigrains

all:
			cargo build --release
			cp $(BINARY_PATH) ./$(BINARY_NAME)

clean:
			cargo clean

fclean:		clean
			rm -f $(BINARY_NAME)

tests_run:	clean
			cargo test
			
re:			fclean all

.PHONY:		all clean fclean re debug test
# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: amejia <amejia@student.42.fr>              +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2024/02/10 18:04:55 by amejia            #+#    #+#              #
#    Updated: 2024/03/19 21:39:58 by amejia           ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = predicter
NAME1 = trainer
SRCS = main1.rs main2.rs linear_regression.rs
SRCS1 = main2.rs
ARTIFACTS = predicted_v_real.html residuals.html scatter.html betas.csv


all: $(NAME) $(NAME1)
	@RUSTFLAGS="-D warnings" cargo build --release
	@cp target/release/$(NAME) .
	@cp target/release/$(NAME1) .
	
debug: 
	cargo build 
	@cp target/debug/$(NAME) .
	@cp target/debug/$(NAME1) .


$(NAME): src/$(SRCS)
$(NAME1): src/$(SRCS1)
	
	
	

src/$(SRCS):
	

clean:
	@cargo clean

fclean: clean
	@rm -f $(NAME)
	@rm -f $(NAME1)
	@rm -f $(ARTIFACTS)

re: fclean all

run: all
	@./$(NAME)

.PHONY: all clean fclean re run

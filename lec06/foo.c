bool have_i_done_the_thing = false;

void foo(void) {
	if (!have_i_done_the_thing) {
		do_the_thing();

		have_i_done_the_thing = true;
	}
}

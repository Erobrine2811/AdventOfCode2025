DAY?=1
CODERS?=
PART?=
DAYS?=

.PHONY: new analyze

new:
	venv/bin/python ./scripts/new_coder.py

analyze:
ifeq ($(strip $(DAYS)),)
	venv/bin/python ./scripts/analyze.py --day $(DAY) $(if $(CODERS),--coders "$(CODERS)") $(if $(PART),--part $(PART))
else
	venv/bin/python ./scripts/analyze.py --days "$(DAYS)" $(if $(CODERS),--coders "$(CODERS)") $(if $(PART),--part $(PART))
endif

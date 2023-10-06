VERSION 0.7


e2e-formatting-base:
		FROM python:3.12.0-slim
		COPY "./ci" "./ci"
		COPY "./unreferenced_files/end-to-end-tests" "./unreferenced_files/end-to-end-tests"
		RUN pip3 install -r "./unreferenced_files/end-to-end-tests/autopep8.requirements.txt"


check-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/check-e2e-formatting.sh


fix-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/fix-e2e-formatting.sh
		SAVE ARTIFACT "./unreferenced_files/end-to-end-tests" AS LOCAL "./unreferenced_files/end-to-end-tests"

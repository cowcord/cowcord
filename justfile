# update error codes and error groups
[group('update')]
update-errors:
    ./scripts/update/error-codes.sh

# update protobuf definitions for user settings
[group('update')]
[group('protos')]
update-protos-user:
    ./scripts/update/protos/user.sh

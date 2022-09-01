if [ $# -lt 2 ]
then
  echo "Usage: $0 [LEVEL] [PROB_ID]"
  exit 1
fi;

LEVEL=$1
PROB_ID=$2

# Create a directory for the problem
mkdir -p $LEVEL/$PROB_ID
cd $LEVEL/$PROB_ID
cargo init --bin --edition 2021 --name solution


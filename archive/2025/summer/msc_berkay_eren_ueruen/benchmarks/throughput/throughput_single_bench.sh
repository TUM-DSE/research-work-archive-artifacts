#!/bin/bash

# Function to display usage information
# TODO: Fill in options
function usage() {
    echo "Usage: $0 -m model1,model2,... -n n_to_gen1,n_to_gen2, -p prompt_file"
	echo "Prompt_file contains 1 prompt on every line"
	echo "This script runs the throughput benchmark using a single request."
    exit 1  
}

# Parse command line arguments
# getopts is a bash built-in for processing command-line options
# "m:p:n:" means we expect options -m, -p, and -n, each with a required argument (indicated by the colon)
while getopts "m:p:n:" opt; do
    case $opt in
        m) IFS=',' read -ra MODELS <<< "$OPTARG"    # Split comma-separated -m argument into MODELS array
           ;;
        p) IFS=',' read -ra PROMPT_FILE <<< "$OPTARG"    
           ;;
        n) IFS=',' read -ra TOKENS_TO_GENERATE <<< "$OPTARG"  
           ;;
        *) usage ;;  # Call usage() if an unknown option is provided
    esac
done

# Check if all required parameters are provided
# ${#ARRAY[@]} gives the length of the array
if [ ${#MODELS[@]} -eq 0 ] || [ ${#PROMPT_FILE[@]} -ne 1 ] || [ ${#TOKENS_TO_GENERATE[@]} -eq 0 ]; then
    echo "Error: All parameters (-m, -p, -n) are required."
    usage  # Show usage and exit if any parameter is missing
fi

# Trim whitespace from array elements
# This removes leading and trailing spaces from each element
for i in "${!MODELS[@]}"; do  # ${!ARRAY[@]} returns the indices of the array
    MODELS[$i]=$(echo "${MODELS[$i]}" | xargs)  # xargs with no arguments trims whitespace
done

for i in "${!PROMPT_FILE[@]}"; do
    PROMPT_FILE[$i]=$(echo "${PROMPT_FILE[$i]}" | xargs)
done

for i in "${!TOKENS_TO_GENERATE[@]}"; do
    TOKENS_TO_GENERATE[$i]=$(echo "${TOKENS_TO_GENERATE[$i]}" | xargs)
done


# Prepare folders for results
mkdir -p results
native_server_file="results/native_server.txt"
vm_server_file="results/vm_server.txt"
llm_os_file="results/llm_os.txt"
:> $native_server_file 
:> $vm_server_file 
:> $llm_os_file 

nb_prompts=`cat ${PROMPT_FILE[0]} | wc -l`
echo "${PROMPT_FILE[0]} contains ${nb_prompts} prompts."
run=1
total_size=`expr 3 \* ${#MODELS[@]} \* ${nb_prompts} \* ${#TOKENS_TO_GENERATE[@]}`
echo "Running all combinations:"
for model in "${MODELS[@]}"; do
	for prompt_nb in $(seq 1 ${nb_prompts}); do
        for n_to_gen in "${TOKENS_TO_GENERATE[@]}"; do
			
		#	echo "Prompt nb: ${prompt_nb}"
			# extract the `prompt_nb`th line from the file
			prompt=`sed -n "${prompt_nb}p" ${PROMPT_FILE[0]}`

			# Run the native llama-server
			################################################
			command_native="../../llama.cpp/llama-server --metrics -m ${model}"
			echo "[$run/$total_size] Executing: $command_native"
			run=`expr $run + 1`
			eval $command_native &> /dev/null &
			sleep 10
			eval "curl --request POST --url http://localhost:8080/completion --data '{\"prompt\": \"${prompt}\",\"n_predict\": ${n_to_gen}}'"  &> /dev/null
			echo "--------- NATIVE --------- " >> $native_server_file
			eval "curl --request GET --url http://localhost:8080/metrics" >> $native_server_file
			echo "model=$model" >> $native_server_file
			echo "-------------------------- " >> $native_server_file
			pkill -f "llama-server"
			#################################################

			# Run llama-server in VM
			#################################################
			command_vm=""	
			echo "[$run/$total_size] Executing in VM: $command_native"
			run=`expr $run + 1`
			
			# start VM
			cd vm_scripts
			nixos-shell server.nix &
			PID=$!

			# Connect to the VM and start the server
			while ! ssh -o StrictHostKeyChecking=no -p 2345 root@localhost 'echo "Server VM live"'
			do
				sleep 3
			done

			ssh_command="/vm_scripts/throughput_single_vm.sh"
			while ! ssh -o StrictHostKeyChecking=no -p 2345 root@localhost \"$ssh_command\" >> ../$vm_server_file
			do
				sleep 1
			done
			# TODO: How do I find out when to kill VM?
			sleep 20
			pkill -f "qemu-kvm"
			exit 1
			#kill $PID
			cd ..
		
			#################################################
			
			# Run LLM-OS
			#################################################
            command_llmos="./script_helper -m \"$path\" -p \"$text\" -n \"$number\""
			echo "[$run/$total_size] Executing: $command_llmos"
			run=`expr $run + 1`
           # eval $command  # Execute the constructed command using eval
			#################################################
        done
    done
done

echo "All combinations completed."

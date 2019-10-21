## Python script for data extraction

There is a python bulk script to extract the encrypted information from the MongoDB, the Blockchain blocks are stored in the MongoDB database **sensors** on the collection 
**sensors_val$

This script is present in **DDL/mongodb/script_extractor**, it will create a folder in the same place called **dumps_mongo** with all the blockchain decrypted blocks.

The script shows this output when execute: **python bulk_blockchain.py**:

```
Options for bulk_blockchain.py:

     -type: Mandatory option to choose between databases
         simple: Bulk to files the data from the standard sensors blockchain (output path ~/dumps_mongo/simple_blockchain)

     Optional params:

         -start-date: Start date to search with format dd-mm-YYY HH:mm:ss
         -end-date: End date to search with format dd-MM-YYY HH:mm:ss

         **No date parameters mean to get the full data timeline

Example:
           python bulk_blockchain.py -type=simple

           python bulk_blockchain.py -type=simple -start-date="29-07-2019" -end-date="31-07-2019"
```

Extract all the Blockchain:

```
python bulk_blockchain.py -type=simple
```

Search between two dates over the Blockchain:

```
python bulk_blockchain.py -type=simple -start-date="29-07-2019" -end-date="31-07-2019"
